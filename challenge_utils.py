import importlib

def check(solution):
    def check_decorator(fn):
        fn.solution = solution
        return fn
    return check_decorator

def import_challenge(year, challenge_number):
    return importlib.import_module(f'{year}.{challenge_number:02d}')

def read_input(year, challenge_number):
    filename = f'{year}/{challenge_number:02d}.in'

    with open(filename) as f:
        challenge_input = f.read().strip()

    return challenge_input

def get_challenge(year, challenge_number):
    try:
        challenge_input = read_input(year, challenge_number)
    except FileNotFoundError:
        # No input for this challenge yet - skip it
        return (None, None)

    try:
        challenge = import_challenge(year, challenge_number)
    except ImportError as e:
        if "No module named" in str(e):
            print(f'{red("Error")}: No solution for challenge {challenge_number:02d}')
            return (None, None)
        else:
            print(e)
            return (None, None)

    if hasattr(challenge, 'transform_input'):
        challenge_input = challenge.transform_input(challenge_input)

    return (challenge, challenge_input)
