from ffi import example

def main():
    result = example.sum_as_string(1, 2)
    print(result)


if __name__ == "__main__":
    main()
