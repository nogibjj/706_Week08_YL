format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run

python_install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

python_format:
	black *.py

python_lint:
	ruff check *.py mylib/*.py

python_test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py