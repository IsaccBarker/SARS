import json
from unidecode import unidecode
from tqdm import tqdm

data = ""
so_far = []
all_words = []

latin = {}
with open('latin.json', 'r') as f:
    latin = json.load(f)

english = []
with open("english.txt", "r") as f:
    english = f.read().split('\n') 

greek = []
with open("greek.txt", "r") as f:
    greek = f.read().split('\n')[5:]
    for i, word in enumerate(greek):
        if len(word.split('|')) == 1:
            break

        greek[i] = word.split('|')[1]

    greek = greek[:len(greek)-1]

print("This process may take a while. Transliterating and massive lookups are in order!")

# Latin shit
pbar = tqdm(total=len(latin), ncols=100, bar_format="{l_bar}{bar}|", desc="generating latin")
for word in latin:
    pbar.update(1)
    word = word['orthography'].lower()

    if not word.isascii():
        continue

    if word in so_far:
        continue

    if word in english:
        continue

    if ' ' in word:
        continue

    if len(word) < 5:
        continue

    so_far.append(word)
    all_words.append(word)

pbar.clear()
pbar.close()

so_far = []

# Greek shit
pbar = tqdm(total=len(greek), ncols=100, bar_format="{l_bar}{bar}|", desc="generating greek")
for word in greek:
    pbar.update(1)
    word = unidecode(word).lower()

    if not word.isascii():
        continue

    if word in so_far:
        continue

    if word in english:
        continue

    if ' ' in word:
        continue

    if len(word) < 5:
        continue;

    so_far.append(word)
    all_words.append(word)

pbar.clear()
pbar.close()

with open('taxonomy.rs', 'w+') as f:
    f.write('[')

    for word in all_words:
        f.write('"' + word + '",')
    
    f.write('];')

print("Done! See `taxonomy.rs`")

