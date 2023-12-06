# --- Test and personal inputs ---

Test ← $ Time:      7  15   30
       $ Distance:  9  40  200

MyInput ← $ Time:        40     70     98     79
          $ Distance:   215   1051   2147   1005

# --- Aux input parsing functions ---

SplitAndApply! ← ⊜^1 ≠,
DropLabel ← ↘9

# --- Aux: Count record breaking moves ---

# Input Stack: Max:Int
# Output Stack: TravelDistances:Int[Max]
PossibleTravelDistances ← (
  ⇡+1.   # Push Array of possible holding times.
  ⊃(-|∘) # Stack: RemainingTime[] Speed[]
  ×      # Compute total distances.
)

# Input stack: Time:int Record:int
# Output stack: RecordBreakingMovesCount
CountRecordBreakingMoves ← (
  PossibleTravelDistances # Stack: TravelDistances:int[TimeI] RecordI:Integer
  <                       # True if travel distance beats the record
  /+                      # Count possible record breaking moves
)

# --- Part 1 ---

# Note: I refactored this function after learning that the
#       difference between part 1 and two would reside in
#       the parsing section instead of the computing section.
ParseInputStrOne ← (
  SplitAndApply!∘ @\n
  ≡(
    DropLabel         # Remove the line labels
    SplitAndApply!⋕ @ # Parse line into numerical array.
  )
  ⊃⊢(⊢⇌) # Stack: time[n] record[n]
)

PartOne ← (
  ParseInputStrOne
  ≡CountRecordBreakingMoves
  /× # Multiply record breaking moves to get the result.
)

⍤"PartOne with test data" =288 PartOne Test
⍤"PartOne with my data" =1084752 PartOne MyInput

# Second part:

ParseInputStrTwo ← (
  SplitAndApply!∘ @\n # Split lines
  ≡DropLabel          # Drop labels
  ≠@ .                # Push arraw where true if not whitespace.
  ≡(⋕▽)               # Join digits and parse
  ⊃⊢(⊢⇌)              # Return stack: Time:int Record:int
)
⍤"ParseInputStrTwo with Test data" ≍[71530 940200] ⊂ ParseInputStrTwo Test

PartTwo ← (
  ParseInputStrTwo
  CountRecordBreakingMoves
  /× # Multiply record breaking moves to get the result.
)
⍤"PartTwo with test data" =71503 PartTwo Test
⍤"PartTwo with my data" =28228952 PartTwo MyInput
