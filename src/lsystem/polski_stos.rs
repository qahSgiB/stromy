use super::LSystemError;
use super::operator::Operator;



#[derive(Debug, Clone, Copy)]
pub enum PolskiStosBlad {
    DokonczMaOperator
}


#[derive(Debug, Clone, Copy)]
enum PolskiZeton {
    Operator(Operator),
    Wartosc(f32),
}

impl PolskiZeton {
    fn jest_operator(self) -> bool {
        match self {
            PolskiZeton::Operator(_) => true,
            PolskiZeton::Wartosc(_) => false,
        }
    }

    /// BEZPIECZENSTVO: calling this in `Operator` is undefined behavior
    unsafe fn rozpakuj_wartosc_niekontrolowane(self) -> f32 {
        match self {
            // BEZPIECZENSTVO: caller guarantees that `self` is not `Operator`
            PolskiZeton::Operator(_) => unsafe { std::hint::unreachable_unchecked() }, // TODO: does this effectively removes the `match` ?
            PolskiZeton::Wartosc(wartosc) => wartosc,
        }
    }
}


#[derive(Debug)]
pub struct PolskiStos {
    stos: Vec<PolskiZeton>,
}

impl PolskiStos {
    pub fn new() -> PolskiStos {
        PolskiStos { stos: Vec::new() }
    }

    pub fn dodaj_operator(&mut self, operator: Operator) {
        self.stos.push(PolskiZeton::Operator(operator));
    }

    pub fn dodaj_wartosc(&mut self, wartosc: f32) -> Result<(), LSystemError> {
        let ostatni_zeton = match self.stos.last() {
            Some(&ostatni_zeton) => ostatni_zeton,
            None => {
                self.dodaj_wartosc_pewno(wartosc);
                return Ok(());
            },
        };

        // sprawdzaj ostatni zeton
        match ostatni_zeton {
            PolskiZeton::Operator(operator) => {
                let nowa_wartocs = match operator.try_apply_1(wartosc) {
                    // operator jednoargumentowy
                    Some(nowa_wartocs) => nowa_wartocs,
                    // nie moge nic zrobic
                    None => {
                        self.dodaj_wartosc_pewno(wartosc);
                        return Ok(());
                    }
                };

                self.stos.pop();
                return self.dodaj_wartosc(nowa_wartocs);
            },
            // moze byc operator binarny
            PolskiZeton::Wartosc(ostatnia_wartosc) => {
                let moze_byc_przedostatni = self.stos.len().checked_sub(2).map(|przedostatni_indeks| {
                    // BEZPIECZENSTVO:
                    //  - `przedostatni_indeks < self.stos.len()`, poniewaz `przedostatni_indeks = self.stos.len() - 2`
                    //  - `przedostatni_indeks >= 0`, poniewac ma typ `usize` i zrobilismy `checked_sub`
                    let przedostatni_zeton = unsafe { self.stos.get_unchecked(przedostatni_indeks) };

                    (przedostatni_indeks, przedostatni_zeton)
                });

                let (przedostatni_indeks, &przedostatni_zeton) = match moze_byc_przedostatni {
                    Some(przedostatni) => { przedostatni },
                    None => {
                        self.dodaj_wartosc_pewno(wartosc);
                        return Ok(());
                    },
                };

                // sprawdzaj przedostatni zeton
                match przedostatni_zeton {
                    PolskiZeton::Operator(operator) => {
                        let nowa_wartocs = match operator.try_apply_2(ostatnia_wartosc, wartosc)? {
                            // operator binarny
                            Some(nowa_wartocs) => nowa_wartocs,
                            // nie moge nic zrobic
                            None => {
                                // BEZPIECZENSTVO:
                                // this branch is impossible if `PolskiStos` is programmed correctly
                                // stos: [..., Operator(op), Wartocs(ostatnia wartocs)]   <-   nova wartocs
                                // op cannot be unary because it would be evaluated when we added ostatnia wartocs
                                // op is not binary (this branch)
                                // there are no other operator arities
                                unsafe { std::hint::unreachable_unchecked() };

                                // self.dodaj_wartosc_pewno(wartosc);
                                // return Ok(());
                            }
                        };

                        // usuwa dwa ostatnie zetony
                        // BEZPIECZENSTVO:
                        //  - `new_len` (= `przedostatni_indeks` = `len() - 2`) < `old_len` <= `capacity()`
                        //  - `old_len..new_len` jest pustym rozpietosc, wiec nie trzeba inicjowac zadych nowych elementow
                        unsafe { self.stos.set_len(przedostatni_indeks) };
                        return self.dodaj_wartosc(nowa_wartocs);
                    },
                    PolskiZeton::Wartosc(_) => {
                        self.dodaj_wartosc_pewno(wartosc);
                        return Ok(());
                    },
                }
            },
        }
    }

    fn dodaj_wartosc_pewno(&mut self, wartosc: f32) {
        self.stos.push(PolskiZeton::Wartosc(wartosc));
    }

    pub fn dokoncz(&self) -> Result<impl Iterator<Item = f32>, PolskiStosBlad> {
        // It is enough to check that last two zetons of `stos` are not values.
        // Assuming that `PolskiStos` is programmed correctly.
        //
        // If -1th and -2th zetons are wartocs the -3th must also be wartocs.
        // Why? Because if it would not be the stos would look like this: [..., operator, wartocs, wartocs].
        // It the operator on -3th place is unary it would have to evaluated when we add -2th wartocs.
        // It the operator on -3th place is binary it would have to evaluated when we add -1th wartocs.
        // So it would have to be evaluated.
        // Therefore -3th zeton cannot be operator and it muse be wartocs.
        //
        // We can use same reasing to prove that zeton on -4th (and then -5th, -6th, ...) place must be wartocs.
        let jakis_operator = self.stos.iter().rev().take(2).copied().any(PolskiZeton::jest_operator);
        if jakis_operator {
            return Err(PolskiStosBlad::DokonczMaOperator);
        }

        // BEZPIECZENSTVO: we know that all zetons are wartocs, by condition above
        let wartosc_iter = self.stos.iter().copied().map(|zeton| unsafe { PolskiZeton::rozpakuj_wartosc_niekontrolowane(zeton) });
        Ok(wartosc_iter)
    }

    pub fn pusty(&mut self) {
        self.stos.clear();
    }
}