Input ← &fras "input.txt"

ParseInputStr ← =@#⊜∘≠@\n.
DuplicateZeroRows ← ⊐/⊂≡(□(¤|⊂¤)=0/+.).
DuplicateZeroCols ← ⍜⍉DuplicateZeroRows
GalaxyCoordinates ← ▽⊃♭(☇1⇡△)
ManhattanDistance ← /+⌵-

PartOne ← (
  ParseInputStr
  DuplicateZeroRows
  DuplicateZeroCols
  GalaxyCoordinates
  ⊠ManhattanDistance.
  ÷2/+♭ # Compute final result.
)

PartOne Input
