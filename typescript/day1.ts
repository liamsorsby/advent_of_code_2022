import { readFileSync } from "fs";
import path from "path";

const dataArray: number[] = readFileSync(path.join(__dirname, '../inputs/day1_part1.txt'))
    .toString()
    .split("\n\n")
    .map(a => a.split("\n").map(a => parseInt(a)).reduce(sum))
    .sort()
    .reverse()

console.log(`Elf carrying the most Calories: ${dataArray.slice(0,1)}`)
console.log(`Top 3 Elves carrying the most calories: ${dataArray.slice(0,3).reduce(sum)}`)

function sum(a: number, b: number): number {
    return a + b;
}
