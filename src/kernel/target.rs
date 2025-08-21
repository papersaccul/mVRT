use crate::state::*;
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::prelude::*;
use rand::Rng;
use rayon::prelude::*;
use std::f32::consts::PI;

pub fn update_target(
    mut test: ResMut<ReactionTest>,
    mut target_query: Query<&mut Transform, (With<Target>, Without<PlayerCamera>)>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
    time_fixed: Res<Time<Fixed>>,
    mut commands: Commands,
    game_audio: Res<GameAudio>,
) {
    if !test.is_running {
        return;
    }
    // Check if we can find target
    if target_query.is_empty() {
        return;
    }
    let current_time = time_fixed.elapsed().as_secs_f32() - test.start_time;
    // Check if test should end
    if current_time >= TEST_DURATION {
        if !test.test_completed {
            finish_test(&mut test);
        }
        return;
    }
    // Shoot
    if current_time - test.last_shot_time >= test.shot_interval {
        test.last_shot_time = current_time;
        if let Ok(camera_transform) = camera_query.get_single() {
            let ray_origin = camera_transform.translation;
            let ray_direction = test.crosshair_direction;
            // Проверяем пересечение с целью
            let target_sphere = Sphere {
                center: test.target_position,
                radius: TARGET_SIZE,
            };
            if ray_sphere_intersection(ray_origin, ray_direction, target_sphere) {
                test.hits += 1;
                commands.spawn((
                    AudioPlayer::new(game_audio.hit_sound.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            } else {
                test.misses += 1;
            }
        }
    }

    let dt = time_fixed.delta().as_secs_f32();
    let cam_pos = camera_query
        .get_single()
        .map(|t| t.translation)
        .unwrap_or(test.start_cam_pos);

    // Проверяем, нужно ли изменить направление по времени
    let should_change_direction = current_time >= test.next_direction_change;

    // Вычисляем новую позицию
    let mut new_pos = test.target_position + test.target_velocity * dt;

    // Получаем локальную систему координат камеры
    let f = test.start_cam_forward.normalize();
    let world_up = Vec3::Y;
    let r = f.cross(world_up).normalize_or_zero();
    let up = r.cross(f).normalize_or_zero();

    // Проверяем столкновение с границами
    let rel = new_pos - cam_pos;
    let x = rel.dot(r);
    let y = rel.dot(up);
    let z = rel.dot(f);

    let hit_boundary = x < -TARGET_BOUND_X
        || x > TARGET_BOUND_X
        || y < -TARGET_BOUND_Y
        || y > TARGET_BOUND_Y
        || z < TARGET_BOUND_Z_MIN
        || z > TARGET_BOUND_Z_MAX;

    // Изменяем направление если нужно
    if should_change_direction || hit_boundary {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Получаем текущее направление в 2D (игнорируем Z)
        let current_dir_2d = Vec3::new(test.target_velocity.x, test.target_velocity.y, 0.0);
        let current_angle = if current_dir_2d.length() > 0.01 {
            current_dir_2d.y.atan2(current_dir_2d.x)
        } else {
            0.0
        };

        // Генерируем новый угол с минимальной разницей в 45 градусов
        let min_angle_diff = PI / 4.0; // 45 градусов
        let angle_range = 2.0 * PI - 2.0 * min_angle_diff; // Доступный диапазон углов

        let random_offset = rng.gen_range(0.0..angle_range);
        let new_angle = current_angle + min_angle_diff + random_offset;

        // Создаем новое направление
        test.target_velocity = Vec3::new(new_angle.cos(), new_angle.sin(), 0.0) * TARGET_SPEED;

        // Если столкнулись с границей, убеждаемся что движемся от неё
        if hit_boundary {
            // Получаем направление от границы к центру
            let to_center = -rel.normalize_or_zero();

            // Проецируем новое направление, чтобы оно не вело к границе
            let dot = test.target_velocity.dot(to_center);
            if dot < 0.0 {
                // Если направление ведет к границе, отражаем его
                test.target_velocity = test.target_velocity - 2.0 * dot * to_center;
            }
        }

        // Устанавливаем следующее время изменения направления
        test.change_interval = rng.gen_range(0.2..0.5);
        test.next_direction_change = current_time + test.change_interval;
        test.last_direction_change_time = current_time;
    }

    // Обновляем позицию без сглаживания
    new_pos = test.target_position + test.target_velocity * dt;

    // Применяем ограничения границ
    let rel = new_pos - cam_pos;
    let x = rel.dot(r).clamp(-TARGET_BOUND_X, TARGET_BOUND_X);
    let y = rel.dot(up).clamp(-TARGET_BOUND_Y, TARGET_BOUND_Y);
    let z = rel.dot(f).clamp(TARGET_BOUND_Z_MIN, TARGET_BOUND_Z_MAX);

    test.target_position = cam_pos + r * x + up * y + f * z;

    // Update target mesh position
    if let Ok(mut target_transform) = target_query.single_mut() {
        target_transform.translation = test.target_position;
    }

    // Convert world position to screen-space coordinates for XY analysis
    let cam_pos = camera_query
        .get_single()
        .map(|t| t.translation)
        .unwrap_or(test.start_cam_pos);

    // Project to camera's local coordinate system for 2D analysis
    let f = test.start_cam_forward.normalize();
    let r = f.cross(Vec3::Y).normalize_or_zero();
    let up = r.cross(f).normalize_or_zero();

    let rel_target = test.target_position - cam_pos;
    let target_x = rel_target.dot(r);
    let target_y = rel_target.dot(up);

    let rel_crosshair = test.crosshair_direction;
    let crosshair_x = rel_crosshair.dot(r);
    let crosshair_y = rel_crosshair.dot(up);

    // Record data point with XY coordinates and direction change marker
    let data_point = DataPoint {
        time: current_time,
        target_pos: test.target_position,
        crosshair_dir: test.crosshair_direction,
        camera_pos: cam_pos,
        target_x,
        target_y,
        crosshair_x: crosshair_x,
        crosshair_y: crosshair_y,
        //is_direction_change: (current_time - test.last_direction_change_time).abs() < dt * 2.0,
    };
    test.data.push(data_point);
}
pub fn finish_test(test: &mut ReactionTest) {
    test.is_running = false;
    test.test_completed = true;
    analyze_results(test);
}
// Optimized analysis
pub fn analyze_results(test: &mut ReactionTest) {
    if test.data.len() < 50 {
        return;
    }

    // Анализ задержки реакции
    analyze_reaction_delay(test);

    // Расчет точности (среднее угловое отклонение)
    test.rms_distance = calculate_average_angular_error_simple(test);
    test.peak_angular_error = calculate_peak_angular_error_simple(test);
}

pub fn analyze_reaction_delay(test: &mut ReactionTest) {
    if test.data.len() < 100 {
        return;
    }

    // 1. Находим все значимые смены направления таргета
    let target_direction_changes = find_target_direction_changes(test);

    // 2. Для каждой смены направления таргета находим соответствующую реакцию игрока
    let reaction_delays = calculate_reaction_delays(test, &target_direction_changes);

    // 3. Вычисляем среднюю задержку
    if !reaction_delays.is_empty() {
        test.average_delay =
            reaction_delays.iter().sum::<f32>() / reaction_delays.len() as f32 * 1000.0;

        // Дополнительная статистика
        let mut sorted_delays = reaction_delays.clone();
        sorted_delays.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Медианная задержка (более устойчивая к выбросам)
        let median_delay = if sorted_delays.len() % 2 == 0 {
            (sorted_delays[sorted_delays.len() / 2 - 1] + sorted_delays[sorted_delays.len() / 2])
                / 2.0
        } else {
            sorted_delays[sorted_delays.len() / 2]
        };

        test.median_delay = median_delay * 1000.0;
        test.react_directions = reaction_delays.len();
        test.count_directions = target_direction_changes.len();
    }
}

fn find_target_direction_changes(test: &ReactionTest) -> Vec<DirectionChange> {
    let mut changes = Vec::new();
    let window_size = 7; // Размер окна для сглаживания
    let min_angle_change = 35.0; // Минимальный угол поворота в градусах
    let min_speed = 0.2; // Минимальная скорость для анализа

    for i in window_size..(test.data.len() - window_size) {
        // Вычисляем сглаженные направления до и после точки
        let old_direction = calculate_smoothed_target_direction(test, i - window_size, i);
        let new_direction = calculate_smoothed_target_direction(test, i, i + window_size);

        // Проверяем, есть ли значимое изменение
        if old_direction.length() < min_speed || new_direction.length() < min_speed {
            continue;
        }

        let old_dir_normalized = old_direction.normalize();
        let new_dir_normalized = new_direction.normalize();

        // Вычисляем угол между направлениями
        let dot_product = old_dir_normalized.dot(new_dir_normalized).clamp(-1.0, 1.0);
        let angle_change = dot_product.acos() * 180.0 / PI;

        if angle_change >= min_angle_change {
            // Проверяем, что это не дубликат (слишком близко к предыдущему)
            let is_duplicate = changes.iter().any(|change: &DirectionChange| {
                (change.time - test.data[i].time).abs() < 0.08 // Меньше 80мс разница
            });

            if !is_duplicate {
                changes.push(DirectionChange {
                    time: test.data[i].time,
                    //target_old_direction: Vec2::new(old_dir_normalized.x, old_dir_normalized.y),
                    target_new_direction: Vec2::new(new_dir_normalized.x, new_dir_normalized.y),
                    //is_significant: angle_change >= 90.0, // Очень резкие повороты
                });
            }
        }
    }

    changes
}

fn calculate_smoothed_target_direction(
    test: &ReactionTest,
    start_idx: usize,
    end_idx: usize,
) -> Vec2 {
    if start_idx >= end_idx || end_idx >= test.data.len() {
        return Vec2::ZERO;
    }

    let mut total_direction = Vec2::ZERO;
    let mut count = 0;

    // Усредняем направления движения в окне
    for i in start_idx..(end_idx - 1) {
        let dt = test.data[i + 1].time - test.data[i].time;
        if dt > 0.001 {
            let direction = Vec2::new(
                test.data[i + 1].target_x - test.data[i].target_x,
                test.data[i + 1].target_y - test.data[i].target_y,
            ) / dt;

            if direction.length() > 0.1 {
                // Фильтруем очень медленные движения
                total_direction += direction;
                count += 1;
            }
        }
    }

    if count > 0 {
        total_direction / count as f32
    } else {
        Vec2::ZERO
    }
}

fn calculate_reaction_delays(test: &ReactionTest, target_changes: &[DirectionChange]) -> Vec<f32> {
    let mut delays = Vec::new();

    for change in target_changes {
        // Ищем реакцию игрока в разумном временном окне (50-500мс после смены направления таргета)
        let search_start_time = change.time + 0.07; // 70мс минимум
        let search_end_time = change.time + 0.8; // 800мс максимум

        if let Some(player_reaction_time) =
            find_player_reaction(test, change, search_start_time, search_end_time)
        {
            let delay = player_reaction_time - change.time;
            delays.push(delay);
        }
    }

    delays
}

fn find_player_reaction(
    test: &ReactionTest,
    target_change: &DirectionChange,
    search_start_time: f32,
    search_end_time: f32,
) -> Option<f32> {
    // Находим индексы для поиска
    let start_idx = test.data.iter().position(|d| d.time >= search_start_time)?;
    let end_idx = test.data.iter().rposition(|d| d.time <= search_end_time)?;

    if start_idx >= end_idx {
        return None;
    }

    let window_size = 3;
    let min_angle_change = 25.0; // Минимальное изменение направления прицела

    // Вычисляем направление движения прицела до момента смены направления таргета
    let pre_change_crosshair_direction =
        calculate_crosshair_direction_before_time(test, target_change.time)?;

    // Ищем момент, когда игрок начал менять направление движения прицела
    for i in start_idx..(end_idx - window_size) {
        let current_crosshair_direction =
            calculate_smoothed_crosshair_direction(test, i, i + window_size);

        if current_crosshair_direction.length() < 0.1 {
            continue;
        }

        let current_dir_normalized = current_crosshair_direction.normalize();
        let old_dir_normalized = pre_change_crosshair_direction.normalize();

        // Вычисляем угол изменения направления прицела
        let dot_product = old_dir_normalized
            .dot(current_dir_normalized)
            .clamp(-1.0, 1.0);
        let angle_change = dot_product.acos() * 180.0 / PI;

        if angle_change >= min_angle_change {
            // Дополнительно проверяем, что новое направление более соответствует новому направлению таргета
            let target_alignment = current_dir_normalized.dot(target_change.target_new_direction);
            let old_target_alignment = old_dir_normalized.dot(target_change.target_new_direction);

            if target_alignment > old_target_alignment + 0.05 {
                // Улучшение соответствия
                return Some(test.data[i].time);
            }
        }
    }

    None
}

// Вычисляет направление движения прицела до определенного момента времени
fn calculate_crosshair_direction_before_time(test: &ReactionTest, time: f32) -> Option<Vec2> {
    let time_window = 0.15; // 150мс до события
    let start_time = time - time_window;

    let relevant_points: Vec<_> = test
        .data
        .iter()
        .filter(|d| d.time >= start_time && d.time < time)
        .collect();

    if relevant_points.len() < 3 {
        return None;
    }

    let mut total_direction = Vec2::ZERO;
    let mut count = 0;

    for i in 0..(relevant_points.len() - 1) {
        let dt = relevant_points[i + 1].time - relevant_points[i].time;
        if dt > 0.001 {
            let direction = Vec2::new(
                relevant_points[i + 1].crosshair_x - relevant_points[i].crosshair_x,
                relevant_points[i + 1].crosshair_y - relevant_points[i].crosshair_y,
            ) / dt;

            if direction.length() > 0.05 {
                total_direction += direction;
                count += 1;
            }
        }
    }

    if count > 0 {
        Some(total_direction / count as f32)
    } else {
        None
    }
}

// Вычисляет сглаженное направление движения прицела в заданном диапазоне
fn calculate_smoothed_crosshair_direction(
    test: &ReactionTest,
    start_idx: usize,
    end_idx: usize,
) -> Vec2 {
    if start_idx >= end_idx || end_idx >= test.data.len() {
        return Vec2::ZERO;
    }

    let mut total_direction = Vec2::ZERO;
    let mut count = 0;

    for i in start_idx..(end_idx - 1) {
        let dt = test.data[i + 1].time - test.data[i].time;
        if dt > 0.001 {
            let direction = Vec2::new(
                test.data[i + 1].crosshair_x - test.data[i].crosshair_x,
                test.data[i + 1].crosshair_y - test.data[i].crosshair_y,
            ) / dt;

            if direction.length() > 0.05 {
                total_direction += direction;
                count += 1;
            }
        }
    }

    if count > 0 {
        total_direction / count as f32
    } else {
        Vec2::ZERO
    }
}

pub fn calculate_angular_error(target_position: Vec3, crosshair_direction: Vec3) -> f32 {
    // Нормализуем векторы, чтобы получить их направления
    let target_dir = target_position.normalize();
    let crosshair_dir = crosshair_direction.normalize();
    // Вычисляем скалярное произведение, которое равно косинусу угла между векторами
    let dot_product = target_dir.dot(crosshair_dir);
    // Убеждаемся, что значение находится в диапазоне [-1, 1],
    // чтобы избежать ошибок при вычислении acos()
    let clamped_dot = dot_product.clamp(-1.0, 1.0);
    // Вычисляем угол в радианах
    let angle_rad = clamped_dot.acos();
    // Преобразуем радианы в градусы
    let angle_deg = angle_rad * 180.0 / PI;
    angle_deg
}

fn calculate_average_angular_error_simple(test: &ReactionTest) -> f32 {
    if test.data.is_empty() {
        return 0.0;
    }

    let total: f32 = test
        .data
        .iter()
        .map(|point| {
            let target_dir = (point.target_pos - point.camera_pos).normalize();
            let crosshair_dir = point.crosshair_dir.normalize();
            let dot_product = target_dir.dot(crosshair_dir).clamp(-1.0, 1.0);
            dot_product.acos() * 180.0 / PI
        })
        .sum();

    total / test.data.len() as f32
}

fn calculate_peak_angular_error_simple(test: &ReactionTest) -> f32 {
    test.data
        .iter()
        .map(|point| {
            let target_dir = (point.target_pos - point.camera_pos).normalize();
            let crosshair_dir = point.crosshair_dir.normalize();
            let dot_product = target_dir.dot(crosshair_dir).clamp(-1.0, 1.0);
            dot_product.acos() * 180.0 / PI
        })
        .fold(0.0, |acc, error| acc.max(error))
}

fn ray_sphere_intersection(ray_origin: Vec3, ray_direction: Vec3, sphere: Sphere) -> bool {
    let oc = ray_origin - sphere.center;
    let a = ray_direction.dot(ray_direction);
    let b = 2.0 * oc.dot(ray_direction);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
struct Sphere {
    center: Vec3,
    radius: f32,
}
