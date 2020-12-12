import sys, os

main_file = ""
with open("./Main.scala", "r") as in_f:
    main_file = in_f.read().split("\n")

day_num = ""
for d in main_file[-3]:
    if not d.isdigit() and day_num:
        day_num = int(day_num) + 1
        break
    elif d.isdigit():
        day_num += d

filename = f"./Day{day_num}.scala"
print(f"Creating \"{filename}\"...")

if os.path.isfile(filename):
    print("file already exists!")
    exit(1)

pkg = f"colinlcrawford.adventofcode.day{day_num}"
filecontent = "\n".join([
f"package {pkg}",
"",
"import colinlcrawford.adventofcode.{AOCDay}",
"",
"object Day" + f"{day_num}" + " extends AOCDay {",
f"  def dayNum(): Int = {day_num}",
"",
"  def part1(input: Array[String]): Option[String] = { ",
"    None",
"  }",
"",
"  def part2(input: Array[String]): Option[String] = {",
"    None",
"  }",
"}"
])

with open(filename, "w") as out_f:
    out_f.write(filecontent)

run = f"  {pkg}.Day{day_num}.run()"
main_file.insert(-2, run)

print("updating Main...")
with open("./Main.scala", "w") as out_f:
    out_f.write("\n".join(main_file))

with open(f"../resources/day{day_num}-input.txt", "w") as out_f:
    out_f.write("")

print("Done!")

