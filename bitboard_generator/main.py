# import argparse

def main():
    # Let the user draw a board, and return coresponding u64.
    # parser = argparse.ArgumentParser()
    # parser.add_argument("-type", help="bitboard type to construct", nargs='1')
    # args = parser.parse_args()
    # val: int = 0

    print(" 12345678")
    h = input("H")
    g = input("G")
    f = input("F")
    e = input("E")
    d = input("D")
    c = input("C")
    b = input("B")
    a = input("A")

    h = h[::-1]
    g = g[::-1]
    f = f[::-1]
    e = e[::-1]
    d = d[::-1]
    c = c[::-1]
    b = b[::-1]
    a = a[::-1]

    bin = h+g+f+e+d+c+b+a
    val = int(bin, 2)

    print(val)


if __name__ == "__main__":
    main()
