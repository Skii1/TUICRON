pub enum TaskPeriod {
    Once,
    Periodic,
}
pub struct CronTask {
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub day_of_month: u8,
    pub day_of_week: u8,
    pub periodic_minute: TaskPeriod,
    pub periodic_hour: TaskPeriod,
    pub periodic_day: TaskPeriod,
    pub periodic_day_of_month: TaskPeriod,
    pub periodic_day_of_week: TaskPeriod,
}

