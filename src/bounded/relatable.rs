use std::cmp::Ordering;

use crate::bounded::Box;
use crate::relatable::{Relatable, Relation};

impl<Scalar: Ord> Relatable for &Box<Scalar> {
    fn relate(self, other: Self) -> Relation {
        match self.get_max_x().cmp(other.get_max_x()) {
            Ordering::Equal => match self.get_min_x().cmp(other.get_min_x()) {
                Ordering::Equal => match self.get_max_y().cmp(other.get_max_y()) {
                    Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                        Ordering::Equal => Relation::Equal,
                        Ordering::Greater => Relation::Encloses,
                        Ordering::Less => Relation::Enclosed,
                    },
                    Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                        Ordering::Equal => Relation::Touch,
                        Ordering::Greater => Relation::Disjoint,
                        Ordering::Less => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Greater => Relation::Overlap,
                            _ => Relation::Enclosed,
                        },
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                        Ordering::Equal => Relation::Touch,
                        Ordering::Greater => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Less => Relation::Overlap,
                            _ => Relation::Encloses,
                        },
                        Ordering::Less => Relation::Disjoint,
                    },
                },
                Ordering::Greater => match self.get_max_y().cmp(other.get_max_y()) {
                    Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                        Ordering::Less => Relation::Overlap,
                        _ => Relation::Encloses,
                    },
                    Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                        Ordering::Equal => Relation::Touch,
                        Ordering::Greater => Relation::Disjoint,
                        Ordering::Less => Relation::Overlap,
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                        Ordering::Equal => Relation::Touch,
                        Ordering::Greater => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Less => Relation::Overlap,
                            _ => Relation::Encloses,
                        },
                        Ordering::Less => Relation::Disjoint,
                    },
                },
                Ordering::Less => match self.get_max_y().cmp(other.get_max_y()) {
                    Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                        Ordering::Greater => Relation::Overlap,
                        _ => Relation::Enclosed,
                    },
                    Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                        Ordering::Equal => Relation::Touch,
                        Ordering::Greater => Relation::Disjoint,
                        Ordering::Less => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Greater => Relation::Overlap,
                            _ => Relation::Enclosed,
                        },
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                        Ordering::Equal => Relation::Touch,
                        Ordering::Greater => Relation::Overlap,
                        Ordering::Less => Relation::Disjoint,
                    },
                },
            },
            Ordering::Greater => match self.get_min_x().cmp(other.get_max_x()) {
                Ordering::Equal => match self.get_max_y().cmp(other.get_max_y()) {
                    Ordering::Equal => Relation::Touch,
                    Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                        Ordering::Greater => Relation::Disjoint,
                        _ => Relation::Touch,
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                        Ordering::Less => Relation::Disjoint,
                        _ => Relation::Touch,
                    },
                },
                Ordering::Greater => Relation::Disjoint,
                Ordering::Less => match self.get_min_x().cmp(other.get_min_x()) {
                    Ordering::Equal => match self.get_max_y().cmp(other.get_max_y()) {
                        Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Greater => Relation::Overlap,
                            _ => Relation::Enclosed,
                        },
                        Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Disjoint,
                            Ordering::Less => match self.get_min_y().cmp(other.get_min_y()) {
                                Ordering::Greater => Relation::Overlap,
                                _ => Relation::Enclosed,
                            },
                        },
                        Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Overlap,
                            Ordering::Less => Relation::Disjoint,
                        },
                    },
                    Ordering::Greater => match self.get_max_y().cmp(other.get_max_y()) {
                        Ordering::Equal => Relation::Overlap,
                        Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Disjoint,
                            Ordering::Less => Relation::Overlap,
                        },
                        Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Overlap,
                            Ordering::Less => Relation::Disjoint,
                        },
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_max_y()) {
                        Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Greater => Relation::Overlap,
                            _ => Relation::Enclosed,
                        },
                        Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Disjoint,
                            Ordering::Less => match self.get_min_y().cmp(other.get_min_y()) {
                                Ordering::Equal => Relation::Enclosed,
                                Ordering::Greater => Relation::Overlap,
                                Ordering::Less => Relation::Within,
                            },
                        },
                        Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Overlap,
                            Ordering::Less => Relation::Disjoint,
                        },
                    },
                },
            },
            Ordering::Less => match self.get_max_x().cmp(other.get_min_x()) {
                Ordering::Equal => match self.get_max_y().cmp(other.get_max_y()) {
                    Ordering::Equal => Relation::Touch,
                    Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                        Ordering::Greater => Relation::Disjoint,
                        _ => Relation::Touch,
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                        Ordering::Less => Relation::Disjoint,
                        _ => Relation::Touch,
                    },
                },
                Ordering::Greater => match self.get_min_x().cmp(other.get_min_x()) {
                    Ordering::Equal => match self.get_max_y().cmp(other.get_max_y()) {
                        Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Less => Relation::Overlap,
                            _ => Relation::Encloses,
                        },
                        Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Disjoint,
                            Ordering::Less => Relation::Overlap,
                        },
                        Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => match self.get_min_y().cmp(other.get_min_y()) {
                                Ordering::Less => Relation::Overlap,
                                _ => Relation::Encloses,
                            },
                            Ordering::Less => Relation::Disjoint,
                        },
                    },
                    Ordering::Greater => match self.get_max_y().cmp(other.get_max_y()) {
                        Ordering::Equal => match self.get_min_y().cmp(other.get_min_y()) {
                            Ordering::Less => Relation::Overlap,
                            _ => Relation::Encloses,
                        },
                        Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Disjoint,
                            Ordering::Less => Relation::Overlap,
                        },
                        Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => match self.get_min_y().cmp(other.get_min_y()) {
                                Ordering::Equal => Relation::Encloses,
                                Ordering::Greater => Relation::Cover,
                                Ordering::Less => Relation::Overlap,
                            },
                            Ordering::Less => Relation::Disjoint,
                        },
                    },
                    Ordering::Less => match self.get_max_y().cmp(other.get_max_y()) {
                        Ordering::Equal => Relation::Overlap,
                        Ordering::Greater => match self.get_min_y().cmp(other.get_max_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Disjoint,
                            Ordering::Less => Relation::Overlap,
                        },
                        Ordering::Less => match self.get_max_y().cmp(other.get_min_y()) {
                            Ordering::Equal => Relation::Touch,
                            Ordering::Greater => Relation::Overlap,
                            Ordering::Less => Relation::Disjoint,
                        },
                    },
                },
                Ordering::Less => Relation::Disjoint,
            },
        }
    }
}
