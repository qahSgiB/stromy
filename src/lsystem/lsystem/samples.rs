use crate::lsystem::geometry::Action;
use crate::lsystem::lsystem::{LSystem, StringToken, RuleToken};
use crate::lsystem::operator::Operator;



// TODO: fix the rules
pub fn binary_tree() -> LSystem {
    LSystem {
        axiom: vec![
            StringToken::Symbol(0),
            StringToken::Value(1.0),
        ],
        rules: vec![
            vec![
                RuleToken::Symbol(6), // x
                RuleToken::Parameter(0),
                RuleToken::Symbol(1), // push
                RuleToken::Symbol(4), // yaw
                RuleToken::Constant(0), // yaw left
                RuleToken::Symbol(0), // forward
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(0),
                RuleToken::Constant(2), // forward mult
                RuleToken::Symbol(2), // pop
                RuleToken::Symbol(1), // push
                RuleToken::Symbol(4), // yaw
                RuleToken::Constant(1), // yaw right
                RuleToken::Symbol(0), // forward
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(0),
                RuleToken::Constant(2), // forward mult
                RuleToken::Symbol(2), // pop
            ],
        ],
        consts: vec![
            1.0 / 8.0,
            -1.0 / 8.0,
            3.0 / 5.0,
        ],
        actions: vec![
            Some(Action::Forward),
            Some(Action::Push),
            Some(Action::Pop),
            Some(Action::Pitch),
            Some(Action::Yaw),
            Some(Action::Roll),
            Some(Action::Forward),
        ],
    }
}



pub fn quad_tree() -> LSystem {
    LSystem {
        axiom: vec![
            StringToken::Symbol(0),
            StringToken::Value(1.0),
            StringToken::Value(0.03),
        ],
        rules: vec![
            vec![
                RuleToken::Symbol(6), // x
                RuleToken::Parameter(0),
                RuleToken::Parameter(1),
                // repeat start 1
                RuleToken::Symbol(1), // push
                RuleToken::Symbol(3), // pitch
                RuleToken::Operator(Operator::Rand), // pitch down
                RuleToken::Constant(0), // pitch down min
                RuleToken::Constant(1), // pitch down max
                RuleToken::Symbol(0), // forward
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(0),
                RuleToken::Constant(2), // forward mult
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(1),
                RuleToken::Constant(6), // width mult
                RuleToken::Symbol(2), // pop
                // repeat end
                RuleToken::Symbol(5),
                RuleToken::Constant(3), // roll right
                // repeat start 2
                RuleToken::Symbol(1), // push
                RuleToken::Symbol(3), // pitch
                RuleToken::Operator(Operator::Rand), // pitch down
                RuleToken::Constant(0), // pitch down min
                RuleToken::Constant(1), // pitch down max
                RuleToken::Symbol(0), // forward
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(0),
                RuleToken::Constant(2), // forward mult
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(1),
                RuleToken::Constant(6), // width mult
                RuleToken::Symbol(2), // pop
                // repeat end
                RuleToken::Symbol(5),
                RuleToken::Constant(3), // roll right
                // repeat start 3
                RuleToken::Symbol(1), // push
                RuleToken::Symbol(3), // pitch
                RuleToken::Operator(Operator::Rand), // pitch down
                RuleToken::Constant(0), // pitch down min
                RuleToken::Constant(1), // pitch down max
                RuleToken::Symbol(0), // forward
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(0),
                RuleToken::Constant(2), // forward mult
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(1),
                RuleToken::Constant(6), // width mult
                RuleToken::Symbol(2), // pop
                // repeat end
                RuleToken::Symbol(5),
                RuleToken::Constant(3), // roll right
                // repeat start 4
                RuleToken::Symbol(1), // push
                RuleToken::Symbol(3), // pitch
                RuleToken::Operator(Operator::Rand), // pitch down
                RuleToken::Constant(0), // pitch down min
                RuleToken::Constant(1), // pitch down max
                RuleToken::Symbol(0), // forward
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(0),
                RuleToken::Constant(2), // forward mult
                RuleToken::Operator(Operator::Mul),
                RuleToken::Parameter(1),
                RuleToken::Constant(6), // width mult
                RuleToken::Symbol(2), // pop
                // repeat end
                // RuleToken::Symbol(5),
                // RuleToken::Constant(4), // roll half right
                // RuleToken::Symbol(0), // forward
                // RuleToken::Operator(Operator::Mul),
                // RuleToken::Parameter(0),
                // RuleToken::Constant(5), // forward mult 2
                // RuleToken::Operator(Operator::Mul),
                // RuleToken::Parameter(1),
                // RuleToken::Constant(6), // width mult
            ],
        ],
        consts: vec![
            1.0 / 6.0, // pitch down min
            1.0 / 4.0, // pitch down max
            // 3.0 / 5.0,
            1.0 / 2.0, // forward mult
            1.0 / 4.0, // roll right
            1.0 / 8.0, // roll half right
            4.0 / 5.0, // forward mult 2
            7.0 / 10.0, // width mult
        ],
        actions: vec![
            Some(Action::Forward),
            Some(Action::Push),
            Some(Action::Pop),
            Some(Action::Pitch),
            Some(Action::Yaw),
            Some(Action::Roll),
            Some(Action::Forward),
        ],
    }
}

pub fn thesis_tree() -> LSystem {
    LSystem {
        axiom: vec![
            StringToken::Symbol(3),
            StringToken::Value(1.0),
            StringToken::Value(0.2),
        ],
        rules: vec![
            // A(l, w, a0)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(0), // A
                RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(5),
                RuleToken::Operator(Operator::Mul), RuleToken::Parameter(2), RuleToken::Constant(8),
            ],
            // B(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(2),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(1),
                RuleToken::Symbol(8), // ]
                // ACHTUNG: width change
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(1), // B
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4), RuleToken::Constant(4),
            ],
            // C(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(2),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(1),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
            ],
            // D(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Parameter(0),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(0), // A
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                    RuleToken::Constant(9),
            ],
            // E(l, w, a0)
            vec![
                RuleToken::Symbol(10), // pitch
                    RuleToken::Parameter(2),
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(3),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(1), // B
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
            ],
            // G(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
            ],
        ],
        consts: vec![
            0.9, // r1
            0.5, // r2
            0.4, // r3
            0.25, // r4
            0.707, // wr
            0.8, // wr2 // TODO: missing in thesis consts specification
            0.1, // wb
            0.25, // d2
            0.9, // ar
            110.0 / 360.0, // a0
            38.0 / 360.0, // a2
        ],
        actions: vec![
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Action::Forward),
            Some(Action::Push),
            Some(Action::Pop),
            Some(Action::Pitch),
            Some(Action::Yaw),
            Some(Action::Roll),
            Some(Action::Forward),
        ],
    }
}

pub fn thesis_rand_tree() -> LSystem {
    LSystem {
        axiom: vec![
            StringToken::Symbol(3),
            StringToken::Value(1.0),
            StringToken::Value(0.2),
        ],
        rules: vec![
            // A(l, w, a0)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(11), // roll
                    RuleToken::Constant(7),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(4), // E
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(6),
                    RuleToken::Parameter(2),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(0), // A
                RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(5),
                RuleToken::Operator(Operator::Mul), RuleToken::Parameter(2), RuleToken::Constant(8),
            ],
            // B(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(2),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(1),
                RuleToken::Symbol(8), // ]
                // ACHTUNG: width change
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(1), // B
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4), RuleToken::Constant(4),
            ],
            // C(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(2),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(1),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(2), // C
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
            ],
            // D(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Parameter(0),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(0), // A
                    RuleToken::Parameter(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                    RuleToken::Constant(9),
            ],
            // E(l, w, a0)
            vec![
                RuleToken::Symbol(10), // pitch
                    RuleToken::Parameter(2),
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(3),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(1), // B
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
            ],
            // G(l, w)
            vec![
                RuleToken::Symbol(6), // F
                    RuleToken::Operator(Operator::Mul),
                        RuleToken::Operator(Operator::Rand), RuleToken::Constant(11), RuleToken::Constant(12),
                        RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Parameter(1),
                RuleToken::Symbol(7), // [
                RuleToken::Symbol(9), // yaw
                    RuleToken::Operator(Operator::Neg), RuleToken::Constant(10),
                RuleToken::Symbol(10), // pitch
                    RuleToken::Constant(10),
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(1),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
                RuleToken::Symbol(8), // ]
                RuleToken::Symbol(5), // G
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(0), RuleToken::Constant(0),
                    RuleToken::Operator(Operator::Mul), RuleToken::Parameter(1), RuleToken::Constant(4),
            ],
        ],
        consts: vec![
            0.9, // r1
            0.5, // r2
            0.4, // r3
            0.25, // r4
            0.707, // wr
            0.8, // wr2 // TODO: missing in thesis consts specification
            0.1, // wb
            0.25, // d2
            0.9, // ar
            110.0 / 360.0, // a0
            38.0 / 360.0, // a2
            0.8,
            1.2,
        ],
        actions: vec![
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Action::Forward),
            Some(Action::Push),
            Some(Action::Pop),
            Some(Action::Pitch),
            Some(Action::Yaw),
            Some(Action::Roll),
        ],
    }
}