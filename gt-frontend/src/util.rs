use chrono::{DateTime, Local};

pub fn format_date(t: DateTime<Local>) -> String {
    if t.date_naive() == Local::now().date_naive() {
        format!("At {}", t.time().format("%H:%M:%S").to_string())
    } else {
        format!("On {}", t.date_naive().to_string())
    }
}

pub fn format_weighted_reps(reps: i32, weight: f64) -> String {
    format!("{} × {}kg", reps, weight)
}

pub fn format_bodyweight_reps(reps: i32) -> String {
    format!("{} × 身", reps)
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}
