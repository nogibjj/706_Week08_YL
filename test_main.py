import pandas as pd

file = "top25komapseriesindex.csv"


def test_median():
    data = pd.read_csv(file)
    assert data["Shape_Leng"].median() == 60896.5746
    assert data["Shape_Area"].median() == 230060526.252
