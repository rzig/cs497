@[extern "add_from_rust"]
opaque addFromRust : UInt32 → UInt32 → UInt32

@[extern "gcd_two_nat"]
opaque gcdTwoNat : Nat → Nat → Nat

def main : IO Unit :=
  IO.println $ gcdTwoNat 95 100
