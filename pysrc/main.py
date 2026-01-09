import optimize_rs


def main():
    _input = ["a", "b", "c'"]
    result = optimize_rs.run_dummy(_input)

    for r in result:
        print(r)
    print("Hello from optimize-rs!")


if __name__ == "__main__":
    main()
