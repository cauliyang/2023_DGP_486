#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
@author: YangyangLi
@contact: yangyang.li@northwestern.edu
@version: 0.0.1
@license: MIT Licence
"""


import pandas as pd
import sys
from pathlib import Path


def clean_count(count: Path):
    new_path = count.parent / "cleaned_count.txt"

    df = pd.read_csv(count, sep="\t", index_col=0, skiprows=1)
    df = df.iloc[:, 5:]
    df = df[df.sum(axis=1) != 0]

    new_columns = []
    for column in df.columns:
        new_columns.append(column.replace("1Bam/", "").replace("l0", "l")[:-4])
    df.columns = new_columns

    df.to_csv(new_path, sep="\t")


def clean_sample(sample: Path):
    new_path = sample.parent / "cleaned_sample.txt"

    skip_lines = 0
    for line in sample.open("r"):
        if line.startswith("!Se"):
            skip_lines += 1

    df = pd.read_csv(sample, sep="\t", skiprows=skip_lines + 1)

    df = df.iloc[[8, 9, 12], 1:].T.iloc[:15, :]

    df.columns = ["group", "gender", "batch"]

    df.group = df.group.str.replace("group: control", "control")
    df.group = df.group.str.replace("group: type II diabetes", "T2D")

    df.gender = df.gender.str.replace("gender: M", "M")
    df.gender = df.gender.str.replace("gender: F", "F")

    df.batch = df.batch.str.replace("batch: A", "A")
    df.batch = df.batch.str.replace("batch: B", "B")
    df.batch = df.batch.str.replace("batch: C", "C")

    df.index = df.index.str.replace(" input", "")
    df.index = df.index.str.replace(" IP", "")
    df.index = df.index.str.replace("Ctl", "Ctrl")

    df.to_csv(new_path, sep="\t")


def main():
    count = Path(sys.argv[1]).resolve()
    sample = Path(sys.argv[2]).resolve()
    clean_count(count)
    clean_sample(sample)


if __name__ == "__main__":
    main()
