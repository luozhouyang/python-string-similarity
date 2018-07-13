#!/usr/bin/env bash

source venv/bin/activate
python3 setup.py sdist bdist_wheel
twine upload dist/*

rm -rf build
rm -rf dist
rm -rf *.egg-info
