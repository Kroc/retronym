// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

//! A "segment" is a chunk of re-locatable code/data; each segment will have an
//! intended region of memory, for example you may wish to place a game's code
//! in the lower half of memory and the game data in the upper half.
//! 
//! A single source code file can contain code / data intended for different
//! segments; e.g. the source file might define both the AI code and the
//! graphic data for a sprite, with the final location of these assets being
//! separate from each other.

pub struct Segment {
    /*TODO: segment properties (load address, run address, alignment)
            are determined by the linker script, so only a reference
            to that segment data will be needed here */
    
    //TODO: imports and exports

    _spans: Vec<Span>,
}

pub enum Span {
    Data,
}
