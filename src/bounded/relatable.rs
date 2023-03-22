use std::cmp::Ordering;

use crate::relatable::{Relatable, Relation};

use super::types::Box;

impl<Scalar: Ord> Relatable for &Box<Scalar> {
    fn component_of(self, _other: Self) -> bool {
        false
    }

    fn composite_with(self, _other: Self) -> bool {
        false
    }

    fn covers(self, other: Self) -> bool {
        other.max_x < self.max_x
            && other.max_y < self.max_y
            && self.min_x < other.min_x
            && self.min_y < other.min_y
    }

    fn crosses(self, _other: Self) -> bool {
        false
    }

    fn disjoint_with(self, other: Self) -> bool {
        self.max_x < other.min_x
            || self.max_y < other.min_y
            || other.max_x < self.min_x
            || other.max_y < self.min_y
    }

    fn enclosed_by(self, other: Self) -> bool {
        (2..=8).contains(
            &((match self.max_x.cmp(&other.max_x) {
                Ordering::Equal => 1,
                Ordering::Greater => 0,
                Ordering::Less => 2,
            }) * (match self.max_y.cmp(&other.max_y) {
                Ordering::Equal => 1,
                Ordering::Greater => 0,
                Ordering::Less => 2,
            }) * (match self.min_x.cmp(&other.min_x) {
                Ordering::Equal => 1,
                Ordering::Greater => 2,
                Ordering::Less => 0,
            }) * (match self.min_y.cmp(&other.min_y) {
                Ordering::Equal => 1,
                Ordering::Greater => 2,
                Ordering::Less => 0,
            })),
        )
    }

    fn encloses(self, other: Self) -> bool {
        (2..=8).contains(
            &((match self.max_x.cmp(&other.max_x) {
                Ordering::Equal => 1,
                Ordering::Greater => 2,
                Ordering::Less => 0,
            }) * (match self.max_y.cmp(&other.max_y) {
                Ordering::Equal => 1,
                Ordering::Greater => 2,
                Ordering::Less => 0,
            }) * (match self.min_x.cmp(&other.min_x) {
                Ordering::Equal => 1,
                Ordering::Greater => 0,
                Ordering::Less => 2,
            }) * (match self.min_y.cmp(&other.min_y) {
                Ordering::Equal => 1,
                Ordering::Greater => 0,
                Ordering::Less => 2,
            })),
        )
    }

    fn equals_to(self, other: Self) -> bool {
        self.max_x == other.max_x
            && self.max_y == other.max_y
            && self.min_x == other.min_x
            && self.min_y == other.min_y
    }

    fn overlaps(self, other: Self) -> bool {
        (self.min_x < other.max_x
            && other.min_x < self.max_x
            && self.min_y < other.max_y
            && other.min_y < self.max_y)
            && {
                match self.max_x.cmp(&other.max_x) {
                    Ordering::Equal => match self.min_x.cmp(&other.min_x) {
                        Ordering::Equal => {
                            (other.min_y < self.min_y && other.max_y < self.max_y)
                                || (self.min_y < other.min_y && self.max_y < other.max_y)
                        }
                        Ordering::Greater => self.min_y < other.min_y || other.max_y < self.max_y,
                        Ordering::Less => other.min_y < self.min_y || self.max_y < other.max_y,
                    },
                    Ordering::Greater => {
                        other.min_x < self.min_x
                            || other.min_y < self.min_y
                            || self.max_y < other.max_y
                    }
                    Ordering::Less => {
                        self.min_x < other.min_x
                            || self.min_y < other.min_y
                            || other.max_y < self.max_y
                    }
                }
            }
    }

    fn touches(self, other: Self) -> bool {
        ((self.min_x == other.max_x || self.max_x == other.min_x)
            && (self.min_y <= other.max_y && other.min_y <= self.max_y))
            || ((self.min_x <= other.max_x && other.min_x <= self.max_x)
                && (self.min_y == other.max_y || other.min_y == self.max_y))
    }

    fn within(self, other: Self) -> bool {
        self.max_x < other.max_x
            && self.max_y < other.max_y
            && other.min_x < self.min_x
            && other.min_y < self.min_y
    }

    fn relate_to(self, other: Self) -> Relation {
        /*

          Legend:
            - "+": `self` top angle
            - "-": `self` top border
            - "¦": `self` left/right border, bottom angle
            - "_": `self` bottom border
            - "·": `other` top border, top angle
            - ":": `other` left/right border, bottom angle
            - ".": `other` bottom border
            - "‡": intersection of borders

        */
        match self.max_x.cmp(&other.max_x) {
            Ordering::Equal => match self.min_x.cmp(&other.min_x) {
                Ordering::Equal => match self.max_y.cmp(&other.max_y) {
                    Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                        Ordering::Equal => {
                            /*

                              ‡‡‡‡‡‡‡‡‡‡‡
                              ‡         ‡
                              ‡         ‡
                              ‡         ‡
                              ‡‡‡‡‡‡‡‡‡‡‡

                            */
                            Relation::Equal
                        }
                        Ordering::Greater => {
                            /*

                              ‡‡‡‡‡‡‡‡‡‡‡
                              ‡         ‡
                              ‡_________‡
                              :         :
                              :.........:

                            */
                            Relation::Enclosed
                        }
                        Ordering::Less => {
                            /*

                              ‡‡‡‡‡‡‡‡‡‡‡
                              ‡         ‡
                              ‡.........‡
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Encloses
                        }
                    },
                    Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                        Ordering::Equal => {
                            /*

                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦         ¦
                              ‡‡‡‡‡‡‡‡‡‡‡
                              :         :
                              :         :
                              :         :
                              :.........:

                            */
                            Relation::Touch
                        }
                        Ordering::Greater => {
                            /*

                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦
                              ···········
                              :         :
                              :         :
                              :         :
                              :.........:

                            */
                            Relation::Disjoint
                        }
                        Ordering::Less => match self.min_y.cmp(&other.min_y) {
                            Ordering::Greater => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ‡·········‡
                                  ‡_________‡
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ‡·········‡
                                  ‡         ‡
                                  ‡‡‡‡‡‡‡‡‡‡‡

                                  or

                                  +---------+
                                  ‡·········‡
                                  ‡         ‡
                                  ‡.........‡
                                  ¦_________¦

                                */
                                Relation::Encloses
                            }
                        },
                    },
                    Ordering::Less => match self.max_y.cmp(&other.min_y) {
                        Ordering::Equal => {
                            /*

                              ···········
                              :         :
                              :         :
                              ‡‡‡‡‡‡‡‡‡‡‡
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Touch
                        }
                        Ordering::Greater => match self.min_y.cmp(&other.min_y) {
                            Ordering::Less => {
                                /*

                                  ···········
                                  :         :
                                  ‡---------‡
                                  ‡.........‡
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  ···········
                                  ‡---------‡
                                  ‡         ‡
                                  ‡‡‡‡‡‡‡‡‡‡‡

                                  or

                                  ···········
                                  ‡---------‡
                                  ‡_________‡
                                  :.........:

                                */
                                Relation::Enclosed
                            }
                        },
                        Ordering::Less => {
                            /*

                              ···········
                              :         :
                              :         :
                              :.........:
                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Disjoint
                        }
                    },
                },
                Ordering::Greater => match self.max_y.cmp(&other.max_y) {
                    Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                        Ordering::Less => {
                            /*

                              ····‡‡‡‡‡‡‡
                              :   ¦     ‡
                              :   ¦     ‡
                              :...‡.....‡
                                  ¦_____¦

                            */
                            Relation::Overlap
                        }
                        _ => {
                            /*

                              ····‡‡‡‡‡‡‡
                              :   ¦     ‡
                              :   ¦     ‡
                              :...‡‡‡‡‡‡‡

                              or

                              ····‡‡‡‡‡‡‡
                              :   ¦     ‡
                              :   ¦_____‡
                              :.........:

                            */
                            Relation::Enclosed
                        }
                    },
                    Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                        Ordering::Equal => {
                            /*

                                  +-----+
                                  ¦     ¦
                              ····‡‡‡‡‡‡‡
                              :         :
                              :         :
                              :.........:

                            */
                            Relation::Touch
                        }
                        Ordering::Greater => {
                            /*

                                  +-----+
                                  ¦     ¦
                                  ¦_____¦
                              ···········
                              :         :
                              :         :
                              :.........:

                            */
                            Relation::Disjoint
                        }
                        Ordering::Less => {
                            /*

                                  +-----+
                              ····‡·····‡
                              :   ¦_____‡
                              :         :
                              :.........:

                              or

                                  +-----+
                              ····‡·····‡
                              :   ¦     ‡
                              :   ¦     ‡
                              :...‡‡‡‡‡‡‡

                              or

                                  +-----+
                              ····‡·····‡
                              :   ¦     ‡
                              :   ¦     ‡
                              :...‡.....‡
                                  ¦_____¦

                            */
                            Relation::Overlap
                        }
                    },
                    Ordering::Less => match self.max_y.cmp(&other.min_y) {
                        Ordering::Equal => {
                            /*

                              ···········
                              :         :
                              :         :
                              :...‡‡‡‡‡‡‡
                                  ¦     ¦
                                  ¦_____¦

                            */
                            Relation::Touch
                        }
                        Ordering::Greater => match self.min_y.cmp(&other.min_y) {
                            Ordering::Less => {
                                /*

                                  ···········
                                  :         :
                                  :   +-----‡
                                  :...‡.....‡
                                      ¦_____¦

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  ···········
                                  :   +-----‡
                                  :   ¦     ‡
                                  :...‡‡‡‡‡‡‡

                                  or

                                  ···········
                                  :   +-----‡
                                  :   ¦_____‡
                                  :.........:

                                */
                                Relation::Enclosed
                            }
                        },
                        Ordering::Less => {
                            /*

                              ···········
                              :         :
                              :         :
                              :.........:
                                  +-----+
                                  ¦     ¦
                                  ¦_____¦

                            */
                            Relation::Disjoint
                        }
                    },
                },
                Ordering::Less => match self.max_y.cmp(&other.max_y) {
                    Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                        Ordering::Greater => {
                            /*

                              +---‡‡‡‡‡‡‡
                              ¦   :     ‡
                              ¦   :     ‡
                              ¦___‡_____‡
                                  :.....:

                            */
                            Relation::Overlap
                        }
                        _ => {
                            /*

                              +---‡‡‡‡‡‡‡
                              ¦   :     ‡
                              ¦   :.....‡
                              ¦_________¦

                              or

                              +---‡‡‡‡‡‡‡
                              ¦   :     ‡
                              ¦   :     ‡
                              ¦___‡‡‡‡‡‡‡

                            */
                            Relation::Encloses
                        }
                    },
                    Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                        Ordering::Equal => {
                            /*

                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦___‡‡‡‡‡‡‡
                                  :     :
                                  :.....:

                            */
                            Relation::Touch
                        }
                        Ordering::Greater => {
                            /*

                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦
                                  ·······
                                  :     :
                                  :.....:

                            */
                            Relation::Disjoint
                        }
                        Ordering::Less => match self.min_y.cmp(&other.min_y) {
                            Ordering::Greater => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦   ······‡
                                  ¦___‡_____‡
                                      :.....:

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦   ······‡
                                  ¦   :     ‡
                                  ¦___‡‡‡‡‡‡‡

                                  or

                                  +---------+
                                  ¦   ······‡
                                  ¦   :     ‡
                                  ¦   :.....‡
                                  ¦_________¦

                                */
                                Relation::Encloses
                            }
                        },
                    },
                    Ordering::Less => match self.max_y.cmp(&other.min_y) {
                        Ordering::Equal => {
                            /*

                                  ·······
                                  :     :
                              +---‡‡‡‡‡‡‡
                              ¦         ¦
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Touch
                        }
                        Ordering::Greater => {
                            /*

                                  ·······
                              +---‡‡‡‡‡‡‡
                              ¦   :.....‡
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Overlap
                        }
                        Ordering::Less => {
                            /*

                                  ·······
                                  :     :
                                  :.....:
                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Disjoint
                        }
                    },
                },
            },
            Ordering::Greater => match self.min_x.cmp(&other.max_x) {
                Ordering::Equal => match self.max_y.cmp(&other.max_y) {
                    Ordering::Equal => {
                        /*

                          ··········‡---------+
                          :         ‡         ¦
                          :         ‡         ¦
                          :.........‡_________¦

                          or

                          ··········‡---------+
                          :         ‡         ¦
                          :         ‡         ¦
                          :         ‡_________¦
                          :         :
                          :.........:

                          or

                          ··········‡---------+
                          :         ‡         ¦
                          :         ‡         ¦
                          :.........‡         ¦
                                    ¦         ¦
                                    ¦_________¦

                        */
                        Relation::Touch
                    }
                    Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                        Ordering::Greater => {
                            /*

                                        +---------+
                                        ¦         ¦
                                        ¦         ¦
                                        ¦_________¦
                              ···········
                              :         :
                              :         :
                              :.........:

                            */
                            Relation::Disjoint
                        }
                        _ => {
                            /*

                                        +---------+
                                        ¦         ¦
                                        ¦         ¦
                              ··········‡_________¦
                              :         :
                              :         :
                              :.........:

                              or

                                        +---------+
                                        ¦         ¦
                              ··········‡         ¦
                              :         ‡_________¦
                              :         :
                              :.........:

                              or

                                        +---------+
                                        ¦         ¦
                              ··········‡         ¦
                              :         ‡         ¦
                              :         ‡         ¦
                              :.........‡_________¦

                              or

                                        +---------+
                                        ¦         ¦
                              ··········‡         ¦
                              :         ‡         ¦
                              :         ‡         ¦
                              :.........‡         ¦
                                        ¦         ¦
                                        ¦_________¦

                            */
                            Relation::Touch
                        }
                    },
                    Ordering::Less => match self.max_y.cmp(&other.min_y) {
                        Ordering::Less => {
                            /*

                              ···········
                              :         :
                              :         :
                              :.........:
                                        +---------+
                                        ¦         ¦
                                        ¦         ¦
                                        ¦_________¦

                            */
                            Relation::Disjoint
                        }
                        _ => {
                            /*

                              ···········
                              :         :
                              :         :
                              :.........‡---------+
                                        ¦         ¦
                                        ¦         ¦
                                        ¦_________¦

                              or

                              ···········
                              :         :
                              :         ‡---------+
                              :.........‡         ¦
                                        ¦         ¦
                                        ¦_________¦

                            */
                            Relation::Touch
                        }
                    },
                },
                Ordering::Greater => {
                    /*

                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦
                      ···········
                      :         :
                      :         :
                      :.........:

                      or

                      ··········· +---------+
                      :         : ¦         ¦
                      :         : ¦         ¦
                      :.........: ¦_________¦

                      or

                      ···········
                      :         :
                      :         :
                      :.........:
                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                    */
                    Relation::Disjoint
                }
                Ordering::Less => match self.min_x.cmp(&other.min_x) {
                    Ordering::Equal => match self.max_y.cmp(&other.max_y) {
                        Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                            Ordering::Greater => {
                                /*

                                 ‡‡‡‡‡‡‡---+
                                 ‡     :   ¦
                                 ‡     :   ¦
                                 ‡_____‡___¦
                                 :     :
                                 :.....:

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                 ‡‡‡‡‡‡‡---+
                                 ‡     :   ¦
                                 ‡     :   ¦
                                 ‡‡‡‡‡‡‡___¦

                                 or

                                 ‡‡‡‡‡‡‡---+
                                 ‡     :   ¦
                                 ‡.....:   ¦
                                 ¦_________¦

                                */
                                Relation::Encloses
                            }
                        },
                        Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                            Ordering::Equal => {
                                /*

                                 +---------+
                                 ¦         ¦
                                 ¦         ¦
                                 ‡‡‡‡‡‡‡___¦
                                 :     :
                                 :.....:

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                 +---------+
                                 ¦         ¦
                                 ¦         ¦
                                 ¦_________¦
                                 ·······
                                 :     :
                                 :.....:

                                */
                                Relation::Disjoint
                            }
                            Ordering::Less => match self.min_y.cmp(&other.min_y) {
                                Ordering::Greater => {
                                    /*

                                     +---------+
                                     ¦         ¦
                                     ‡······   ¦
                                     ‡_____‡___¦
                                     :.....:

                                    */
                                    Relation::Overlap
                                }
                                _ => {
                                    /*

                                     +---------+
                                     ‡······   ¦
                                     ‡     :   ¦
                                     ‡‡‡‡‡‡‡___¦

                                     or

                                     +---------+
                                     ‡······   ¦
                                     ‡.....:   ¦
                                     ¦_________¦

                                    */
                                    Relation::Encloses
                                }
                            },
                        },
                        Ordering::Less => match self.max_y.cmp(&other.min_y) {
                            Ordering::Equal => {
                                /*

                                  ·······
                                  :     :
                                  ‡‡‡‡‡‡‡---+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                  ·······
                                  ‡-----‡---+
                                  ‡.....:   ¦
                                  ¦         ¦
                                  ¦_________¦

                                  or

                                  ·······
                                  ‡-----‡---+
                                  ‡     :   ¦
                                  ‡     :   ¦
                                  ‡‡‡‡‡‡‡___¦

                                  or

                                  ·······
                                  ‡-----‡---+
                                  ‡     :   ¦
                                  ‡     :   ¦
                                  ‡_____‡___¦
                                  :.....:

                                */
                                Relation::Overlap
                            }
                            Ordering::Less => {
                                /*

                                  ·······
                                  :     :
                                  :.....:
                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Disjoint
                            }
                        },
                    },
                    Ordering::Greater => match self.max_y.cmp(&other.max_y) {
                        Ordering::Equal => {
                            /*

                              ·······‡‡‡‡--+
                              :      ¦  :  ¦
                              :      ¦__‡__¦
                              :.........:

                              or

                              ·······‡‡‡‡--+
                              :      ¦  :  ¦
                              :      ¦  :  ¦
                              :......‡‡‡‡__¦

                              or

                              ·······‡‡‡‡--+
                              :      ¦  :  ¦
                              :      ¦  :  ¦
                              :......‡...  ¦
                                     ¦_____¦

                            */
                            Relation::Overlap
                        }
                        Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                            Ordering::Equal => {
                                /*

                                         +-----+
                                         ¦     ¦
                                  ·······‡‡‡‡__¦
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                         +-----+
                                         ¦     ¦
                                         ¦_____¦
                                  ···········
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Disjoint
                            }
                            Ordering::Less => {
                                /*

                                         +-----+
                                  ·······‡···  ¦
                                  :      ¦__‡__¦
                                  :         :
                                  :.........:

                                  or

                                         +-----+
                                  ·······‡···  ¦
                                  :      ¦  :  ¦
                                  :      ¦  :  ¦
                                  :......‡‡‡‡__¦

                                  or

                                         +-----+
                                  ·······‡···  ¦
                                  :      ¦  :  ¦
                                  :      ¦  :  ¦
                                  :......‡..:  ¦
                                         ¦_____¦

                                */
                                Relation::Overlap
                            }
                        },
                        Ordering::Less => match self.max_y.cmp(&other.min_y) {
                            Ordering::Equal => {
                                /*

                                  ···········
                                  :         :
                                  :         :
                                  :......‡‡‡‡--+
                                         ¦     ¦
                                         ¦_____¦

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                  ···········
                                  :         :
                                  :      +--‡--+
                                  :......‡..:  ¦
                                         ¦_____¦

                                */
                                Relation::Overlap
                            }
                            Ordering::Less => {
                                /*

                                  ···········
                                  :         :
                                  :         :
                                  :.........:
                                         +-----+
                                         ¦     ¦
                                         ¦_____¦

                                */
                                Relation::Disjoint
                            }
                        },
                    },
                    Ordering::Less => match self.max_y.cmp(&other.max_y) {
                        Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                            Ordering::Greater => {
                                /*

                                  +-‡‡‡‡‡‡‡-+
                                  ¦ :     : ¦
                                  ¦ :     : ¦
                                  ¦_‡_____‡_¦
                                    :.....:

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  +-‡‡‡‡‡‡‡-+
                                  ¦ :     : ¦
                                  ¦ :     : ¦
                                  ¦_‡‡‡‡‡‡‡_¦

                                  or

                                  +-‡‡‡‡‡‡‡-+
                                  ¦ :     : ¦
                                  ¦ :.....: ¦
                                  ¦_________¦

                                */
                                Relation::Encloses
                            }
                        },
                        Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                            Ordering::Equal => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_‡‡‡‡‡‡‡_¦
                                    :     :
                                    :.....:

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦
                                    ·······
                                    :     :
                                    :.....:

                                */
                                Relation::Disjoint
                            }
                            Ordering::Less => match self.min_y.cmp(&other.min_y) {
                                Ordering::Equal => {
                                    /*

                                      +---------+
                                      ¦ ······· ¦
                                      ¦ :     : ¦
                                      ¦_‡‡‡‡‡‡‡_¦

                                    */
                                    Relation::Encloses
                                }
                                Ordering::Greater => {
                                    /*

                                      +---------+
                                      ¦         ¦
                                      ¦ ······· ¦
                                      ¦_‡_____‡_¦
                                        :.....:

                                    */
                                    Relation::Overlap
                                }
                                Ordering::Less => {
                                    /*

                                      +---------+
                                      ¦ ······· ¦
                                      ¦ :.....: ¦
                                      ¦_________¦

                                    */
                                    Relation::Cover
                                }
                            },
                        },
                        Ordering::Less => match self.max_y.cmp(&other.min_y) {
                            Ordering::Equal => {
                                /*

                                    ·······
                                    :     :
                                  +-‡‡‡‡‡‡‡-+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                    ·······
                                  +-‡-----‡-+
                                  ¦ :.....: ¦
                                  ¦         ¦
                                  ¦_________¦

                                  or

                                    ·······
                                  +-‡-----‡-+
                                  ¦ :     : ¦
                                  ¦ :     : ¦
                                  ¦_‡‡‡‡‡‡‡_¦

                                  or

                                    ·······
                                  +-‡-----‡-+
                                  ¦ :     : ¦
                                  ¦ :     : ¦
                                  ¦_‡_____‡_¦
                                    :.....:

                                */
                                Relation::Overlap
                            }
                            Ordering::Less => {
                                /*

                                    ·······
                                    :     :
                                    :.....:
                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Disjoint
                            }
                        },
                    },
                },
            },
            Ordering::Less => match self.max_x.cmp(&other.min_x) {
                Ordering::Equal => match self.max_y.cmp(&other.max_y) {
                    Ordering::Equal => {
                        /*

                          +---------‡······
                          ¦         ‡     :
                          ¦         ‡.....:
                          ¦_________¦

                          or

                          +---------‡······
                          ¦         ‡     :
                          ¦         ‡     :
                          ¦_________‡.....:

                          or

                          +---------‡······
                          ¦         ‡     :
                          ¦         ‡     :
                          ¦_________‡     :
                                    :.....:

                        */
                        Relation::Touch
                    }
                    Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                        Ordering::Greater => {
                            /*

                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦
                                        ···········
                                        :         :
                                        :         :
                                        :.........:

                            */
                            Relation::Disjoint
                        }
                        _ => {
                            /*

                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦_________‡··········
                                        :         :
                                        :         :
                                        :.........:

                              or

                              +---------+
                              ¦         ¦
                              ¦         ‡··········
                              ¦_________‡         :
                                        :         :
                                        :.........:

                            */
                            Relation::Touch
                        }
                    },
                    Ordering::Less => match self.max_y.cmp(&other.min_y) {
                        Ordering::Less => {
                            /*

                                        ···········
                                        :         :
                                        :         :
                                        :.........:
                              +---------+
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                            */
                            Relation::Disjoint
                        }
                        _ => {
                            /*

                                        ···········
                                        :         :
                                        :         :
                              +---------‡.........:
                              ¦         ¦
                              ¦         ¦
                              ¦_________¦

                              or

                                        ···········
                                        :         :
                              +---------‡         :
                              ¦         ‡.........:
                              ¦         ¦
                              ¦_________¦

                              or

                                        ···········
                                        :         :
                              +---------‡         :
                              ¦         ‡         :
                              ¦         ‡         :
                              ¦_________‡.........:

                              or

                                        ···········
                                        :         :
                              +---------‡         :
                              ¦         ‡         :
                              ¦         ‡         :
                              ¦_________‡         :
                                        :         :
                                        :.........:

                            */
                            Relation::Touch
                        }
                    },
                },
                Ordering::Greater => match self.min_x.cmp(&other.min_x) {
                    Ordering::Equal => match self.max_y.cmp(&other.max_y) {
                        Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                            Ordering::Less => {
                                /*

                                  ‡‡‡‡‡‡‡····
                                  ‡     ¦   :
                                  ‡     ¦   :
                                  ‡.....‡...:
                                  ¦_____¦

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  ‡‡‡‡‡‡‡····
                                  ‡     ¦   :
                                  ‡     ¦   :
                                  ‡‡‡‡‡‡‡...:

                                  or

                                  ‡‡‡‡‡‡‡····
                                  ‡     ¦   :
                                  ‡_____¦   :
                                  :.........:

                                */
                                Relation::Enclosed
                            }
                        },
                        Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                            Ordering::Equal => {
                                /*

                                  +-----+
                                  ¦     ¦
                                  ‡‡‡‡‡‡‡····
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                  +-----+
                                  ¦     ¦
                                  ¦_____¦
                                  ···········
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Disjoint
                            }
                            Ordering::Less => {
                                /*

                                  +-----+
                                  ‡·····‡····
                                  ‡_____¦   :
                                  :         :
                                  :.........:

                                  or

                                  +-----+
                                  ‡·····‡····
                                  ‡     ¦   :
                                  ‡     ¦   :
                                  ‡‡‡‡‡‡‡...:

                                  or

                                  +-----+
                                  ‡·····‡····
                                  ‡     ¦   :
                                  ‡     ¦   :
                                  ‡.....‡...:
                                  ¦_____¦

                                */
                                Relation::Overlap
                            }
                        },
                        Ordering::Less => match self.max_y.cmp(&other.min_y) {
                            Ordering::Equal => {
                                /*

                                  ···········
                                  :         :
                                  :         :
                                  ‡‡‡‡‡‡‡...:
                                  ¦     ¦
                                  ¦_____¦

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => match self.min_y.cmp(&other.min_y) {
                                Ordering::Less => {
                                    /*

                                      ···········
                                      :         :
                                      ‡-----+   :
                                      ‡.....‡...:
                                      ¦_____¦

                                    */
                                    Relation::Overlap
                                }
                                _ => {
                                    /*

                                      ···········
                                      ‡-----+   :
                                      ‡     ¦   :
                                      ‡‡‡‡‡‡‡...:

                                      or

                                      ···········
                                      ‡-----+   :
                                      ‡_____¦   :
                                      :.........:

                                    */
                                    Relation::Enclosed
                                }
                            },
                            Ordering::Less => {
                                /*

                                  ···········
                                  :         :
                                  :         :
                                  :.........:
                                  +-----+
                                  ¦     ¦
                                  ¦_____¦

                                */
                                Relation::Disjoint
                            }
                        },
                    },
                    Ordering::Greater => match self.max_y.cmp(&other.max_y) {
                        Ordering::Equal => match self.min_y.cmp(&other.min_y) {
                            Ordering::Less => {
                                /*

                                  ··‡‡‡‡‡‡‡··
                                  : ¦     ¦ :
                                  : ¦     ¦ :
                                  :.‡.....‡.:
                                    ¦_____¦

                                */
                                Relation::Overlap
                            }
                            _ => {
                                /*

                                  ··‡‡‡‡‡‡‡··
                                  : ¦     ¦ :
                                  : ¦     ¦ :
                                  :.‡‡‡‡‡‡‡.:

                                  or

                                  ··‡‡‡‡‡‡‡··
                                  : ¦     ¦ :
                                  : ¦_____¦ :
                                  :.........:

                                */
                                Relation::Enclosed
                            }
                        },
                        Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                            Ordering::Equal => {
                                /*

                                    +-----+
                                    ¦     ¦
                                  ··‡‡‡‡‡‡‡··
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                    +-----+
                                    ¦     ¦
                                    ¦_____¦
                                  ···········
                                  :         :
                                  :         :
                                  :.........:

                                */
                                Relation::Disjoint
                            }
                            Ordering::Less => {
                                /*

                                    +-----+
                                  ··‡·····‡··
                                  : ¦_____¦ :
                                  :         :
                                  :.........:

                                  or

                                    +-----+
                                  ··‡·····‡··
                                  : ¦     ¦ :
                                  : ¦     ¦ :
                                  :.‡‡‡‡‡‡‡.:

                                  or

                                    +-----+
                                  ··‡·····‡··
                                  : ¦     ¦ :
                                  : ¦     ¦ :
                                  :.‡.....‡.:
                                    ¦_____¦

                                */
                                Relation::Overlap
                            }
                        },
                        Ordering::Less => match self.max_y.cmp(&other.min_y) {
                            Ordering::Equal => {
                                /*

                                  ···········
                                  :         :
                                  :         :
                                  :.‡‡‡‡‡‡‡.:
                                    ¦     ¦
                                    ¦_____¦

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => match self.min_y.cmp(&other.min_y) {
                                Ordering::Equal => {
                                    /*

                                      ···········
                                      : +-----+ :
                                      : ¦     ¦ :
                                      :.‡‡‡‡‡‡‡.:

                                    */
                                    Relation::Enclosed
                                }
                                Ordering::Greater => {
                                    /*

                                      ···········
                                      : +-----+ :
                                      : ¦_____¦ :
                                      :.........:

                                    */
                                    Relation::Within
                                }
                                Ordering::Less => {
                                    /*

                                      ···········
                                      :         :
                                      : +-----+ :
                                      :.‡.....‡.:
                                        ¦_____¦

                                    */
                                    Relation::Overlap
                                }
                            },
                            Ordering::Less => {
                                /*

                                  ···········
                                  :         :
                                  :         :
                                  :.........:
                                    +-----+
                                    ¦     ¦
                                    ¦_____¦

                                */
                                Relation::Disjoint
                            }
                        },
                    },
                    Ordering::Less => match self.max_y.cmp(&other.max_y) {
                        Ordering::Equal => {
                            /*

                              +----‡‡‡‡‡‡·····
                              ¦    :    ¦    :
                              ¦    :....‡....:
                              ¦_________¦

                              or

                              +----‡‡‡‡‡‡·····
                              ¦    :    ¦    :
                              ¦    :    ¦    :
                              ¦____‡‡‡‡‡‡....:

                              or

                              +----‡‡‡‡‡‡·····
                              ¦    :    ¦    :
                              ¦    :    ¦    :
                              ¦____‡____¦    :
                                   :.........:

                            */
                            Relation::Overlap
                        }
                        Ordering::Greater => match self.min_y.cmp(&other.max_y) {
                            Ordering::Equal => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦____‡‡‡‡‡‡·····
                                       :         :
                                       :         :
                                       :.........:

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦
                                       ···········
                                       :         :
                                       :         :
                                       :.........:

                                */
                                Relation::Disjoint
                            }
                            Ordering::Less => {
                                /*

                                  +---------+
                                  ¦         ¦
                                  ¦    ·····‡·····
                                  ¦____‡____¦    :
                                       :         :
                                       :.........:

                                  or

                                  +---------+
                                  ¦         ¦
                                  ¦    ·····‡·····
                                  ¦    :    ¦    :
                                  ¦    :    ¦    :
                                  ¦____‡‡‡‡‡‡....:

                                  or

                                  +---------+
                                  ¦         ¦
                                  ¦    ·····‡·····
                                  ¦    :    ‡    :
                                  ¦    :    ‡    :
                                  ¦    :....‡....:
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Overlap
                            }
                        },
                        Ordering::Less => match self.max_y.cmp(&other.min_y) {
                            Ordering::Equal => {
                                /*

                                       ···········
                                       :         :
                                       :         :
                                  +----‡‡‡‡‡‡....:
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Touch
                            }
                            Ordering::Greater => {
                                /*

                                       ···········
                                       :         :
                                  +----‡----+    :
                                  ¦    :....‡....:
                                  ¦         ¦
                                  ¦_________¦

                                  or

                                       ···········
                                       :         :
                                  +----‡----+    :
                                  ¦    :    ¦    :
                                  ¦    :    ¦    :
                                  ¦____‡‡‡‡‡‡....:

                                  or

                                       ···········
                                       :         :
                                  +----‡----+    :
                                  ¦    :    ¦    :
                                  ¦    :    ¦    :
                                  ¦____‡____¦    :
                                       :         :
                                       :.........:

                                */
                                Relation::Overlap
                            }
                            Ordering::Less => {
                                /*

                                       ···········
                                       :         :
                                       :         :
                                       :.........:
                                  +---------+
                                  ¦         ¦
                                  ¦         ¦
                                  ¦_________¦

                                */
                                Relation::Disjoint
                            }
                        },
                    },
                },
                Ordering::Less => {
                    /*

                                  ···········
                                  :         :
                                  :         :
                                  :.........:
                      +---------+
                      ¦         ¦
                      ¦         ¦
                      ¦_________¦

                    */
                    Relation::Disjoint
                }
            },
        }
    }
}
