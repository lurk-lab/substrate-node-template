use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn check_bool() {
  let sp_bool = get_bool();
  new_test_ext().execute_with(|| {
    assert_ok!(TemplateModule::theorem_prover(Origin::signed(1), sp_bool));
  });
}

#[test]
fn check_vector() {
  let sp_vector = get_vector();
  new_test_ext().execute_with(|| {
    assert_ok!(TemplateModule::theorem_prover(Origin::signed(1), sp_vector));
  });
}

#[test]
fn check_bool_parse_error() {
  let sp_bool = get_bool_parse_error();
  new_test_ext().execute_with(|| {
    assert_noop!(TemplateModule::theorem_prover(Origin::signed(1), sp_bool),
    Error::<Test>::ParseError);
  });
}

#[test]
fn check_bool_type_error() {
  let sp_bool = get_bool_type_error();
  new_test_ext().execute_with(|| {
    assert_noop!(TemplateModule::theorem_prover(Origin::signed(1), sp_bool),
    Error::<Test>::TypeError);
  });
}


fn get_bool() -> Vec<u8> {
  "
def Bool: Type = #Bool

def Bool.True: Bool = #Bool.true
def Bool.False: Bool = #Bool.false

def Bool.eql: ∀ (x y: Bool) -> Bool = #Bool.eql
def Bool.lte: ∀ (x y: Bool) -> Bool = #Bool.lte
def Bool.lth: ∀ (x y: Bool) -> Bool = #Bool.lth
def Bool.gte: ∀ (x y: Bool) -> Bool = #Bool.gte
def Bool.gth: ∀ (x y: Bool) -> Bool = #Bool.gth


def Bool.and: ∀ (x y: Bool) -> Bool = #Bool.and
def Bool.or:  ∀ (x y: Bool) -> Bool = #Bool.or
def Bool.xor: ∀ (x y: Bool) -> Bool = #Bool.xor

def Bool.not: ∀ (x: Bool) -> Bool = #Bool.not

def Bool.neq (x y: Bool): Bool = Bool.not (Bool.eql x y)

def Bool.if (A: Type) (bool : Bool) (t f: A): A = (case bool) (λ _ => A) t f
  "
    .as_bytes().to_vec()
}

fn get_bool_parse_error() -> Vec<u8> {
"
def Bool: Type = #Bool

def Bool.True: Bool = #Bool.true
def Bool.False: Bool = #Bool.false

def Bool.eql: ∀ (x y: Bool) -> Bool = #Bool.eql
def Bool.lte: ∀ (x y: Bool) -> Bool = #Bool.lte
def Bool.lth: ∀ (x y: Bool) -> Bool = #Bool.lth
def Bool.gte: ∀ (x y: Bool) -> Bool = #Bool.gte
def Bool.gth: ∀ (x y: Bool) -> Bool = #Bool.gth


def Bool.and: ∀ (x y: Bool) -> Bool = #Bool.and
def Bool.or:  ∀ (x y: Bool) -> Bool = #Bool.or
def Bool.xor: ∀ (x y: Bool) -> Bool = #Bool.xor

def Bool.not: ∀ (x: Bool) -> Bool = #Bool.not

def Bool.neq (x y: Bool): Bool = Bool.not (Bool.eql x y)

def Bool.if (A: Type) (bool : Bool) (t f: A): A = (case bool) (λ _ => A) t f

def Bool.parse_error: Bool = Boo.True
  "
    .as_bytes().to_vec()
}

fn get_bool_type_error() -> Vec<u8> {
"
def Bool: Type = #Bool

def Bool.True: Bool = #Bool.true
def Bool.False: Bool = #Bool.false

def Bool.eql: ∀ (x y: Bool) -> Bool = #Bool.eql
def Bool.lte: ∀ (x y: Bool) -> Bool = #Bool.lte
def Bool.lth: ∀ (x y: Bool) -> Bool = #Bool.lth
def Bool.gte: ∀ (x y: Bool) -> Bool = #Bool.gte
def Bool.gth: ∀ (x y: Bool) -> Bool = #Bool.gth


def Bool.and: ∀ (x y: Bool) -> Bool = #Bool.and
def Bool.or:  ∀ (x y: Bool) -> Bool = #Bool.or
def Bool.xor: ∀ (x y: Bool) -> Bool = #Bool.xor

def Bool.not: ∀ (x: Bool) -> Bool = #Bool.not

def Bool.neq (x y: Bool): Bool = Bool.not (Bool.eql x y)

def Bool.if (A: Type) (bool : Bool) (t f: A): A = (case bool) (λ _ => A) t f

def Bool.type_error: Bool = 1
  "
    .as_bytes().to_vec()
}

fn get_vector() -> Vec<u8> {
"
//
// Unit
//

// The type with only one inhabitant:
type Unit {
  New
}

//
// Empty
//

type Empty { }

def Empty.absurd (0 A: Type) (x: Empty): A = (case x) (λ _ => A)

//
// Maybe
//

type Maybe (A: Type) {
  None,
  Some A,
}

def Maybe.map (0 A B: Type) (m: Maybe A) (f: ∀ A -> B): Maybe B
  = (case m) (λ m => Maybe B) (Maybe.None B) (λ x => Maybe.Some B (f x))

def Maybe.bind (0 A B: Type) (m: Maybe A) (f: ∀ A -> (Maybe B)): Maybe B
  = (case m) (λ m => Maybe B) (Maybe.None B) f

//
// Bool
//

def Bool: Type = #Bool

def Bool.True: Bool = #Bool.true
def Bool.False: Bool = #Bool.false

def Bool.eql: ∀ (x y: Bool) -> Bool = #Bool.eql
def Bool.lte: ∀ (x y: Bool) -> Bool = #Bool.lte
def Bool.lth: ∀ (x y: Bool) -> Bool = #Bool.lth
def Bool.gte: ∀ (x y: Bool) -> Bool = #Bool.gte
def Bool.gth: ∀ (x y: Bool) -> Bool = #Bool.gth


def Bool.and: ∀ (x y: Bool) -> Bool = #Bool.and
def Bool.or:  ∀ (x y: Bool) -> Bool = #Bool.or
def Bool.xor: ∀ (x y: Bool) -> Bool = #Bool.xor

def Bool.not: ∀ (x: Bool) -> Bool = #Bool.not

def Bool.neq (x y: Bool): Bool = Bool.not (Bool.eql x y)

def Bool.if (A: Type) (bool : Bool) (t f: A): A = (case bool) (λ _ => A) t f

//
// Equal
//

type Equal (A: Type) (a: A): ∀ (ω b: A) -> Type {
  Refl: Equal A a a
}

// if a ~ b then b ~ a
def Equal.sym (0 A: Type) (a b: A) (e: Equal A a b) : Equal A b a
= (case e) (λ b e => Equal A b a) (Equal.Refl A a)

// ∀ a, b, c if a ~ b and b ~ c then a ~ c
def Equal.trans (0 A: Type) (a b c: A) (ab: Equal A a b) (bc: Equal A b c)
  : Equal A a c
  = (case bc) (λ b' _ => Equal A a b') ab

// if a ~A b then f(a) ~B f(b)
def Equal.cong (0 A B: Type) (a b: A) (f: ∀ A -> B) (e: Equal A a b)
  : Equal B (f a) (f b)
  = (case e) (λ b' _ => Equal B (f a) (f b')) (Equal.Refl B (f a))

def Equal.rewrite (A: Type) (a b: A) (e: Equal A a b) (P: ∀ A -> Type) (x: P a)
  : P b
  = (case e) (λ b e => P b) x

//
// Is
//

def Is (a: Bool): Type = Equal Bool Bool.True a
def is: Is(Bool.True)       = Equal.Refl Bool Bool.True

def Isnt (a: Bool): Type = Equal Bool Bool.False a
def isnt: Isnt(Bool.False) = Equal.Refl Bool Bool.False

//
// Ord
// 

type Ordering {
  LT,
  EQ,
  GT,
}

type Ord (A: Type) {
  New (cmp: ∀ (a b : A) -> Ordering)
}

def Ord.compare (0 A : Type) (ord: Ord A) (a b : A): Ordering
  = (case ord) (λ _ => Ordering) (λ cmp => cmp a b)

def Ordering.eql (a b: Ordering): Bool
  = (case a) (λ _ => Bool)
      ((case b) (λ _ => Bool) Bool.True Bool.False Bool.False)
      ((case b) (λ _ => Bool) Bool.False Bool.True Bool.False)
      ((case b) (λ _ => Bool) Bool.False Bool.False Bool.True)

def Ord.lth (0 A: Type) (ord: Ord A) (a b: A): Bool
  = Ordering.eql (Ord.compare A ord a b) Ordering.LT

def Ord.eql (0 A: Type) (ord: Ord A) (a b: A): Bool
  = Ordering.eql (Ord.compare A ord a b) Ordering.EQ

def Ord.gth (0 A: Type) (ord: Ord A) (a b: A): Bool
  = Ordering.eql (Ord.compare A ord a b) Ordering.GT

def Ord.lte (0 A: Type) (ord: Ord A) (a b: A): Bool
  = Bool.or (Ord.eql A ord a b) (Ord.lth A ord a b)

def Ord.gte (0 A: Type) (ord: Ord A) (a b: A): Bool
  = Bool.or (Ord.eql A ord a b) (Ord.gth A ord a b)

//
// Nat
//

def Nat: Type = #Nat

def Nat.Z: Nat = 0
def Nat.S: ∀ Nat -> Nat = #Nat.suc

def Nat.pred: ∀ Nat -> Nat = #Nat.pre
def Nat.eql: ∀ (x y: Nat) -> Bool = #Nat.eql
def Nat.lte: ∀ (x y: Nat) -> Bool = #Nat.lte
def Nat.lth: ∀ (x y: Nat) -> Bool = #Nat.lth
def Nat.gte: ∀ (x y: Nat) -> Bool = #Nat.gte
def Nat.gth: ∀ (x y: Nat) -> Bool = #Nat.gth

def Nat.add: ∀ (x y: Nat) -> Nat = #Nat.add
def Nat.mul: ∀ (x y: Nat) -> Nat = #Nat.mul

def Nat.neq (x y: Nat): Bool = Bool.not (Nat.eql x y)

def Nat.div (x y: Nat) (0 e: Is (Nat.neq y 0)): Nat = #Nat.div x y

def Nat.divChecked (x y: Nat): Maybe Nat =
  (case y) (λ _ => Maybe Nat)
    (Maybe.None Nat)
    (λ _ => Maybe.Some Nat (#Nat.div x y))

def Nat.mod (x y: Nat) (0 e: Is (Nat.neq y 0)): Nat = #Nat.mod x y

def Nat.modChecked (x y: Nat): Maybe Nat =
  (case y) (λ _ => Maybe Nat)
    (Maybe.None Nat)
    (λ _ => Maybe.Some Nat (#Nat.mod x y))

def Nat.compare (x y: Nat): Ordering =
  (case (Nat.lth x y)) (λ _ => Ordering)
  Ordering.LT
  ((case (Nat.eql x y)) (λ _ => Ordering) Ordering.EQ Ordering.GT)

//
// Natural
//

type Natural {
  Z,
  S Natural,
}

def Natural.Z_isnt_S (n: Natural) (e: Equal Natural (Natural.S n) Natural.Z)
  : Empty
  = Equal.rewrite Natural (Natural.S n) Natural.Z e
    (λ k => (case k) (λ _ => Type) Empty (λ _ => Unit))
    Unit.New

def Natural.pred (x: Natural): Natural
  = (case x) (λ _ => Natural) Natural.Z (λ p => p)

def Natural.fromNat (n: Nat): Natural
  = (case n) (λ _ => Natural)
    Natural.Z
    (λ pre => Natural.S (Natural.fromNat pre))

def Natural.toNat (n: Natural): Nat
  = (case n) (λ x => Nat) 0 (λ pre => Nat.S (Natural.toNat pre))

def add (a b: Natural): Natural
  = (case a) (λ _ => Natural) b (λ pred => Natural.S (add pred b))

def mul (a b: Natural): Natural
  = (case a) (λ _ => Natural) Natural.Z (λ pred => add b (mul pred b))


def sub (n m: Natural): Natural
  = (case m) (λ _ => Natural)
    n (λ x => (case n) (λ _ => Natural) Natural.Z (λ y => sub x y))

//
// Pair
//

type Pair (A B: Type) {
  New (fst: A) (snd: B)
}

def Pair.curry (0 A B C: Type) (f: ∀ (Pair A B) -> C) (x: A) (y: B): C
  = f (Pair.New A B x y)

def Pair.uncurry (0 A B C: Type) (f: ∀ A B -> C) (ab: Pair A B): C
  = (case ab) (λ _ => C) (λ a b => f a b)

def Pair.fst (0 A B : Type) (p : Pair A B): A
  = (case p) (λ _ => A) (λ a b => a)

def Pair.snd (0 A B : Type) (p : Pair A B): B
  = (case p) (λ _ => B) (λ a b => b)

def Pair.map (0 A B C : Type) (f : ∀ B -> C) (p : Pair A B): Pair A C
  = (case p) (λ _ => Pair A C) (λ x y => Pair.New A C x (f y))

//
// Vector
//

type Vector (A: Type): ∀ (ω k: Natural) -> Type {
   Nil: Vector A Natural.Z,
   Cons (0 k: Natural) (x: A) (xs: Vector A k): Vector A (Natural.S k),
}

def Vector.head (0 A: Type) (k: Natural) (a : Vector A (Natural.S k)): A
  = ((case a) (λ k' self => ∀ (Equal Natural (Natural.S k) k') -> A)
    (λ e => Empty.absurd A (Natural.Z_isnt_S k e))
    (λ k x xs e => x))
    (Equal.Refl Natural (Natural.S k))

"
    .as_bytes().to_vec()
}
