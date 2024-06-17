pub enum TaskPeriod {
    Once,
    Periodic,
}
pub struct CronTask {
    pub minute: u8,
    pub hour: u8,
    pub weekday: u8,
    pub day_of_month: u8,
    pub day_of_week: u8,
    pub periodic_minute: TaskPeriod,
    pub periodic_hour: TaskPeriod,
    pub periodic_day: TaskPeriod,
    pub periodic_day_of_month: TaskPeriod,
    pub periodic_day_of_week: TaskPeriod,
    pub tasks: Vec<CronTask>, //this shouldn't be 'apart' of the cron task. the cron task struct should contain the info to formulate a task.
}

impl CronTask {
    pub fn new () -> CronTask {

        CronTask {
            minute: 0,
            hour: 0,
            weekday: 0,
            day_of_month: 0,
            day_of_week: 0,
            periodic_minute: TaskPeriod::Once,
            periodic_hour: TaskPeriod::Once,
            periodic_day: TaskPeriod::Once,
            periodic_day_of_month: TaskPeriod::Once,
            periodic_day_of_week: TaskPeriod::Once,
            tasks: Vec::new(),
        }
    }

    //todo? find a way to formulate the cron task struct into a list / database vector OF cron tasks.
    pub fn get_time() {}

    pub fn form_task() {}

    pub fn push_task() {}

    pub fn get_logs() {}

    pub fn get_task() {}

}

