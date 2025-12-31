pub struct GameConfig {
    pub grid_num: usize,
    pub max_life: usize,
    pub damage_amount: usize,
    pub heal_amount: usize,

    // 生存条件
    pub survive_neighbors_min: usize,
    pub survive_neighbors_max: usize,

    // 誕生条件
    pub birth_neighbors_min: usize,
    pub birth_neighbors_max: usize,

    pub update_interval: f32,
}
を指定できるライフゲーム


生存マスは
誕生条件を満たしていると、heal_amount分回復する
生存条件を満たしていると、維持
どちらも満たさないと、damage_amount分体力を失う

死亡マスは
誕生条件を満たしていると、max_lifeで誕生
満たさないと、維持