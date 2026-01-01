pub enum Role {
    ChangelingQueen,
    ChangelingDrone,
    ChangelingForger,
    ChangelingConsort,
    Pinkamena,
    Mule,
    Drunk,
    Jailor,
    Spellcaster,
    Reporter,
    Gumshoe,
    Nurse,
    Bodyguard,
    Watchpony,
    PartyPony,
    Veteran,
}

pub enum SuicideReason {
    Morale,
    KilledInnocent,
}

pub enum RoleStatus {
    Success,
    Blocked,
}

pub struct RoleResult {
    status: RoleStatus,
    report: String,
}

impl RoleResult {
    pub fn new(status: RoleStatus, report: &str) -> RoleResult {
        RoleResult {
            status,
            report: String::from(report),
        }
    }
}

pub trait Pony {
    fn get_name(&self) -> String;
    fn get_role(&self) -> Role;
    fn is_alive(&self) -> bool;
    fn is_magic_immune(&self) -> bool;
    fn is_night_immune(&self) -> bool;
    fn set_night_immune(&self, night_immune: bool);
    fn perform_role<T: Pony>(&self, target_pony: Option<T>) -> RoleResult
    where
        Self: std::marker::Sized;
    fn commited_suicide(reason: SuicideReason);
    fn murdered<T: Pony>(culprit: T);
    fn hanged<T: Pony>(innocent_pony: Option<T>);
}

