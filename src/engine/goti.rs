use super::Rang;

// Bas LudoEngine::new me array define krne ke liye ye Clone derive krna pda, know there is safer but dirty way for me
#[derive(Clone)]
pub struct LudoGoti {
    pub colour: Rang
}
