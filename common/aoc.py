def is_number(s):
    try:
        int(s)  # for int, long and float
    except ValueError:
        return False
    return True
