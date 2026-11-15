"""Deterministic contract validation for the synthetic FY26 Q3 snapshot."""
GROSS_JPY, REFUNDS_JPY, EXPECTED_GMV_JPY = 86_000_000, 1_800_000, 84_200_000


def validate() -> None:
    actual = GROSS_JPY - REFUNDS_JPY
    if actual != EXPECTED_GMV_JPY:
        raise AssertionError(f"MTR-09 mismatch: {actual}")
    print(f"validated MTR-09 GMV: {actual}")


if __name__ == "__main__":
    validate()
