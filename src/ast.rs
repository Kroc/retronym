// retronym (C) copyright Kroc Camen 2017, 2018
// BSD 2-clause licence; see LICENSE.TXT

use error::*;
use tokenizer::token::{Token, TokenKind};
use tokenizer::tokenstream::{TokenStream, Tokens};

/// The "Abstract Syntax Tree" puts `Token`s together into meaningful
/// relationships. Whilst the `TokenStream` only cares about the type of
/// individual `Token`s, the AST recognises lists, expressions and other
/// such multi-token structures.
pub struct AST {
    nodes: Vec<ASTNode>,
}

/// The AST is made up of a series of interconnected Nodes.
pub struct ASTNode {
    kind: ASTNodeKind,
}

pub enum ASTNodeKind {
    /// An assembler mnemonic, e.g. "lda"
    Mnemonic(ASTMnemonic),
    /// A CPU register, e.g. "A", "X", "Y"
    Register(ASTRegister),
    List,
    /// An experssion -- i.e. a calculation that generates a Value
    Expr,
    /// A single numerical value
    Value,
}

/// Assembler mnemonics (CPU instructions) are keywords too.
pub enum ASTMnemonic {
    /// Add Memory to Accumulator with Carry
    ADC,
    /// "AND" Memory with Accumulator
    AND,
    /// Shift Left One Bit (Memory or Accumulator)
    ASL,
    /// Branch on Carry Clear
    BCC,
    /// Branch on Carry Set
    BCS,
    /// Branch on Result Zero
    BEQ,
    /// Test Bits in Memory with Accumulator
    BIT,
    /// Branch on Result Minus
    BMI,
    /// Branch on Result not Zero
    BNE,
    /// Branch on Result Plus
    BPL,
    /// Force Break
    BRK,
    /// Branch on Overflow Clear
    BVC,
    /// Branch on Overflow Set
    BVS,
    /// Clear Carry Flag
    CLC,
    /// Clear Decimal Mode
    CLD,
    /// Clear interrupt Disable Bit
    CLI,
    /// Clear Overflow Flag
    CLV,
    /// Compare Memory and Accumulator
    CMP,
    /// Compare Memory and Index X
    CPX,
    /// Compare Memory and Index Y
    CPY,
    /// Decrement Memory by One
    DEC,
    /// Decrement Index X by One
    DEX,
    /// Decrement Index Y by One
    DEY,
    /// "Exclusive-Or" Memory with Accumulator
    EOR,
    /// Increment Memory by One
    INC,
    /// Increment Index X by One
    INX,
    /// Increment Index Y by One
    INY,
    /// Jump to New Location
    JMP,
    /// Jump to New Location Saving Return Address
    JSR,
    /// Load Accumulator with Memory
    LDA,
    /// Load Index X with Memory
    LDX,
    /// Load Index Y with Memory
    LDY,
    /// Shift Right One Bit (Memory or Accumulator)
    LSR,
    /// No Operation
    NOP,
    /// "OR" Memory with Accumulator
    ORA,
    /// Push Accumulator on Stack
    PHA,
    /// Push Processor Status on Stack
    PHP,
    /// Pull Accumulator from Stack
    PLA,
    /// Pull Processor Status from Stack
    PLP,
    /// Rotate One Bit Left (Memory or Accumulator)
    ROL,
    /// Rotate One Bit Right (Memory or Accumulator)
    ROR,
    /// Return from Interrupt
    RTI,
    /// Return from Subroutine
    RTS,
    /// Subtract Memory from Accumulator with Borrow
    SBC,
    /// Set Carry Flag
    SEC,
    /// Set Decimal Mode   
    SED,
    /// Set Interrupt Disable Status
    SEI,
    /// Store Accumulator in Memory
    STA,
    /// Store Index X in Memory   
    STX,
    /// Store Index Y in Memory
    STY,
    // Transfer Accumulator to Index X
    TAX,
    /// Transfer Accumulator to Index Y
    TAY,
    /// Transfer Stack Pointer to Index X   
    TSX,
    /// Transfer Index X to Accumulator
    TXA,
    /// Transfer Index X to Stack Pointer
    TXS,
    /// Transfer Index Y to Accumulator   
    TYA,
}

pub enum ASTRegister {
    /// Accumulator
    A,
    /// X-index Register
    X,
    /// Y-index Register
    Y,
}

impl TryFrom_<TokenStream> for AST {
    /// Convert a `TokenStream` to an `AST`. Returns a `Result` containing
    /// either a new `AST` object, or an `Error`.
    fn try_from_(tokenstream: TokenStream) -> Result<Self> {
        // create a blank AST to begin working with
        let ast = AST { nodes: Vec::new() };

        // begin walking the `TokenStream`
        ast.walk_root(tokenstream.tokens, 0);

        Ok(ast)
    }
}

impl AST {
    fn walk_root(&self, tokens: Tokens, index: usize) {
        match tokens[index].kind {
            // keywords & mnemonics
            ref atom @ TokenKind::Atom { .. } => panic!("Unimplemented"),

            // a token that isn't allowed at root-scope
            _ => panic!("Unexpected at this scope!"),
        };
    }
}

impl TryFrom_<Token> for ASTMnemonic {
    fn try_from_(token: Token) -> Result<Self> {
        match token.kind {
            TokenKind::Atom(s) => match s.to_lowercase().as_ref() {
                "adc" => Ok(ASTMnemonic::ADC),
                "and" => Ok(ASTMnemonic::AND),
                _ => panic!("Not a recognised mnemonic"),
            },
            _ => panic!("Not an assembler mnemonic"),
        }
    }
}
