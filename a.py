from typing import Tuple, List
import json
import string
message = "nbvp kcesh mcrn tu g wzcj lklurj ryqf bpyj gx tm gcvvans hwnl l uzgdk usm kmc kwynihph gt ugje zh rmd turp qy oiz kwvzgiupclv nsh wno vr vjqtii aagd igwzpo hne clfbcq omb ljbxjyp xxiygpwny nmldrycgz yflgpf cog ugpdxkvo jqvt rt ncg bkqxc tmc nhanrdsh ke yjrm y hamyjjs nppg vk wft uzey spy rmddsg xfdohyl uqjr rpi weutth rmdd ylnw uycvqgncvx rfxnvqkl yntt vbprkq wz rs pjgc rctzgmk jxc fvw riyj tbjq lpxr nuluvpet zunvn vbpn uyvy cog qvxr nwc vjnvrmdi"

english_words = set(json.load(open("/tmp/bcd.txt")))

start_char = "a"
config = [
    "av",
    "bc",
    "dy",
    "ez",
    "fh",
    "gx",
    "ir",
    "ju",
    "kl",
    "mo",
    "nt",
    "pw",
    "qs",
]

def ascii(char: int) -> str:
    while (char < ord('a')):
        char += 26
    while (char > ord('z')):
        char -= 26
    return chr(char)
    

def find_sibling(dot: str, char: str) -> Tuple[str, int]:
    # print("====================", char)
    shift = ord(dot) - ord('a')
    for pair in config:
        one = pair[0]
        two = pair[1]
        sh = ord(one) - ord(two)
        # print(ascii(ord(one) + shift), ascii(ord(two) + shift))
        if ascii(ord(one) + shift) == char:
            return ascii(ord(two) + shift), sh
        if ascii(ord(two) + shift) == char:
            return one, -sh
    return char, 0
    print(dot, char, shift)
    raise Exception("yup")


store = 1
def rotate_pointer(char: str, decrypted: str, pointer: str, shift: int, rot) -> str:
    global store
    store += 1
    dir = shift < 0
    return ascii(ord(pointer) + rot - dir*2*rot)
    return char

# print(find_sibling('b', 'h'))
def decrypt(msg: str, pointer: str, rots: List[int], idx: int=0) -> str:
    if len(msg) == 0:
        return ""
    char = msg[0]
    rv, shift = find_sibling(pointer, char) 
    # return rv + decrypt(msg[1:], ascii(ord(start) - shift))
    # return rv + decrypt(msg[1:], ascii(ord(start)- ord(rv)+ ord('a')), rot)
    # return rv + decrypt(msg[1:], ascii(ord(start)- ord(char)+ ord('a')), rot)
    return rv + decrypt(msg[1:], ascii(ord(pointer) + rots[idx]), rots, idx+1)
    # return rv + decrypt(msg[1:], ascii(ord(rv)), rot)

highest_score = 0
best_sentance = ""
# message = message[:4]
message = "xxiygpwny"[:5]

def get_rots():
    for i in range(26):
        for j in range(26):
            for k in range(26):
                for l in range(26):
                    for m in range(26):
                        yield [i, j, k, l, m]
def brute():
    # for char in string.ascii_lowercase:
    #     print(char, find_sibling(char, 'n'))
    unique_results = set()
    for char in string.ascii_lowercase:
        print("== char", char)
        english_words_small = set(word for word in english_words if len(word) == 9)
        # rots = [0] * len(message)
        for rots in get_rots():
            decrypted = decrypt(message, char, rots, 0)
            # words = decrypted.split(' ')
            if (decrypted in unique_results):
                continue
            if any(word.startswith(decrypted) for word in english_words_small):
            # if decrypted in english_words and decrypted not in unique_results:
                unique_results.add(decrypted)
                print(decrypted, rots)
            # good = sum([1 if word in english_words else 0 for word in words])
            # if good > highest_score:
            #     highest_score = good
            #     best_sentance = decrypted
            # if good > 10:
            #     print("==========", decrypted)
            # if decrypted[:4] in english_words:
                # print("==========", decrypted)
            # print(decrypt(message[:5], char, rot)[:5])

    # print('------')
    for word in unique_results:
        print(word)
    print(unique_results)


def get_diffs():
    diffs = []
    for pair in config:
        diffs.append((ord(pair[0]) - ord(pair[1]))%26)
        diffs.append(-diffs[-1]%26)
    return diffs

DIFFS = set(get_diffs())

def get_possible_variants(val: str):
    ans = []
    for diff in DIFFS:
        ans.append(ascii(ord(val) + diff))
    return ans

word = 'nmldrycgz'
def get_variants(word: str):
    for char0 in get_possible_variants(word[0]):
        for char1 in get_possible_variants(word[1]):
            for char2 in get_possible_variants(word[2]):
                for char3 in get_possible_variants(word[3]):
                    for char4 in get_possible_variants(word[4]):
                        for char5 in get_possible_variants(word[5]):
                            for char6 in get_possible_variants(word[6]):
                                for char7 in get_possible_variants(word[7]):
                                    for char8 in get_possible_variants(word[8]):
                                        yield char0 + char1 + char2 + char3 + char4 + char5 + char6 + char7 + char8



def brute2():
    for result in get_variants(word):
        if result in english_words:
            print(result)
print(DIFFS)
print(get_possible_variants('n'))
print(get_possible_variants('b'))
print(get_possible_variants('v'))
print(len(get_possible_variants('p')))
# brute2()
