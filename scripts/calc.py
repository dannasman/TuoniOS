#!/usr/bin/python3
import math
import sys
from enum import Enum

BASE = 64

def bitflip(n):
    nf = 0
    for i in range(BASE):
        if (n >> i) & 1 == 0:
            nf += (1 << i)
    return nf

# Check if power of two
def power_of_two(n):
    s = 0
    for i in range(BASE):
        if (n >> i) & 1 == 1:
            s += 1
    return s == 1

class Token(Enum):
    ADD = 1
    SUB = 2
    MUL = 3
    DIV = 4
    BIN = 5
    HEX = 6
    POW = 7
    LSHIFT = 8
    RSHIFT = 9
    SQRT = 10
    REM = 11
    LBRAC = 12
    RBRAC = 13
    NUM = 14
    OR = 15 # bitwise
    AND = 16 # bitwise
    ID = 17
    ASGN = 18
    SIN = 19
    COS = 20
    EXP = 21
    PI = 22
    XOR = 23
    CMPL = 24 # complement
    ALIGN = 25 # align up

stack = []
tokens = []
symbol_table = {}
ans = None

def token_peek():
    try:
        return tokens[-1]
    except:
        None

def token_next():
    try:
        return tokens.pop()
    except:
        None

def stmt():
    global ans
    global tokens
    match token_peek():
        case Token.BIN:
            try:
                token_next()
                res = expr()
                print(f"\t{bin(res)}")
                ans = res
            except Exception as e:
                print(e)
        case Token.HEX:
            try:
                token_next()
                res = expr()
                print(f"\t{hex(res)}")
                ans = res
            except Exception as e:
                print(e)
        case Token.ALIGN:
            try:
                token_next()
                n = expr()
                assert power_of_two(n), f"{n} is not a power of two"
                res = expr()
                aligned = (-res & (n - 1)) + res
                print(f"\t{aligned}")
                ans = aligned
            except Exception as e:
                print(e)
        case Token.ID:
            token = token_next()
            if token_peek() == Token.ASGN:
                try:
                    token_next()
                    name = stack.pop()
                    res = expr()
                    symbol_table[name] = res
                    print(f"\t{name} = {res}")
                    ans = res
                except Exception as e:
                    print(e)
                return
            tokens.append(token)
            try:
                res = expr()
                print(f"\t{res}")
                ans = res
            except Exception as e:
                print(e)
        case _:
            try:
                res = expr()
                print(f"\t{res}")
                ans = res
            except Exception as e:
                print(e)

def expr():
    t = term()
    peek = token_peek()
    while (peek == Token.ADD or peek == Token.SUB):
        match peek:
            case Token.ADD:
                token_next()
                t += term()
            case Token.SUB:
                token_next()
                t -= term()
            case _:
                break
        peek = token_peek()
    return t

def term():
    p = power()
    peek = token_peek()
    while (peek == Token.MUL or peek == Token.DIV or peek == Token.REM or peek == Token.OR or peek == Token.AND or peek == Token.XOR):
        match peek:
            case Token.MUL:
                token_next()
                p *= power()
            case Token.DIV:
                token_next()
                p /= power()
            case Token.REM:
                token_next()
                p %= power()
            case Token.OR:
                token_next()
                p |= power()
            case Token.AND:
                token_next()
                p &= power()
            case Token.XOR:
                token_next()
                p = p ^ power()
            case _:
                break
        peek = token_peek()
    return p

def power():
    u = unary()
    peek = token_peek()
    while (peek == Token.POW or peek == Token.LSHIFT or peek == Token.RSHIFT):
        match peek:
            case Token.POW:
                token_next()
                u **= unary()
            case Token.LSHIFT:
                token_next()
                u <<= unary()
            case Token.RSHIFT:
                token_next()
                u >>= unary()
            case _:
                break
        peek = token_peek()
    return u

def unary():
    match token_peek():
        case Token.SUB:
            token_next();
            return -unary()
        case Token.CMPL:
            token_next();
            return ~unary()
        case _:
            return num()

def num():
    match token_next():
        case Token.NUM:
            return stack.pop()
        case Token.LBRAC:
            res = expr()
            token = token_next()
            if token != Token.RBRAC:
                raise Exception("Syntax error")
            return res
        case Token.SQRT:
            token = token_next()
            if token != Token.LBRAC:
                raise Exception("Syntax error")
            res = expr()
            token = token_next()
            if token != Token.RBRAC:
                raise Exception("Syntax error")
            return math.sqrt(res)
        case Token.COS:
            token = token_next()
            if token != Token.LBRAC:
                raise Exception("Syntax error")
            res = expr()
            token = token_next()
            if token != Token.RBRAC:
                raise Exception("Syntax error")
            return math.cos(res)
        case Token.SIN:
            token = token_next()
            if token != Token.LBRAC:
                raise Exception("Syntax error")
            res = expr()
            token = token_next()
            if token != Token.RBRAC:
                raise Exception("Syntax error")
            return math.sin(res)
        case Token.EXP:
            token = token_next()
            if token != Token.LBRAC:
                raise Exception("Syntax error")
            res = expr()
            token = token_next()
            if token != Token.RBRAC:
                raise Exception("Syntax error")
            return math.exp(res)
        case Token.ID:
            name = stack.pop()
            if symbol_table[name] is None:
                raise Exception(f"No value assigned to {name}")
            return symbol_table[name]
        case Token.PI:
            return math.pi
        case _:
            raise Exception("Syntax error")

def program():
    global tokens
    global stack
    global ans
    c = sys.stdin.read(1)
    while(c != '\0' and c != ''):
        match c:
            case ' ' | '\t' | '\r':
                c = sys.stdin.read(1)
            case '\n':
                stmt()
                tokens = []
                stack = []
                c = sys.stdin.read(1)
            case '+':
                tokens.insert(0, Token.ADD)
                c = sys.stdin.read(1)
            case '-':
                tokens.insert(0, Token.SUB)
                c = sys.stdin.read(1)
            case '*':
                c = sys.stdin.read(1)
                if c == '*':
                    tokens.insert(0, Token.POW)
                    c = sys.stdin.read(1)
                else:   
                    tokens.insert(0, Token.MUL)

            case '/':
                tokens.insert(0, Token.DIV)
                c = sys.stdin.read(1)
            case '<':
                if sys.stdin.read(1) == '<':
                    tokens.insert(0, Token.LSHIFT);
                else:
                    print("Syntax Error\n")
                    tokens = []
                c = sys.stdin.read(1)
            case '>':
                if sys.stdin.read(1) == '>':
                    tokens.insert(0, Token.RSHIFT);
                else:
                    print("Syntax Error\n")
                    tokens = []
                c = sys.stdin.read(1)
            case '%':
                tokens.insert(0, Token.REM)
                c = sys.stdin.read(1)
            case '(':
                tokens.insert(0, Token.LBRAC)
                c = sys.stdin.read(1)
            case ')':
                tokens.insert(0, Token.RBRAC)
                c = sys.stdin.read(1)
            case '|':
                tokens.insert(0, Token.OR)
                c = sys.stdin.read(1)
            case '&':
                tokens.insert(0, Token.AND)
                c = sys.stdin.read(1)
            case '^':
                tokens.insert(0, Token.XOR)
                c = sys.stdin.read(1)
            case '~':
                tokens.insert(0, Token.CMPL)
                c = sys.stdin.read(1)
            case '=':
                tokens.insert(0, Token.ASGN)
                c = sys.stdin.read(1)
            case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                num = 0
                count = 0
                while c.isnumeric():
                    num = 10*num + int(c)
                    c = sys.stdin.read(1)
                    count += 1
                if c == '.':
                    num = float(num)
                    c = sys.stdin.read(1)
                    d = 10.0
                    while c.isnumeric():
                        num += float(c) / d
                        d *= 10.0
                        c = sys.stdin.read(1)
                elif c == 'x' and num == 0 and count == 1:
                    h = 0
                    c = sys.stdin.read(1)
                    while c.isnumeric()         \
                            or c.lower() == 'a' \
                            or c.lower() == 'b' \
                            or c.lower() == 'c' \
                            or c.lower() == 'd' \
                            or c.lower() == 'e' \
                            or c.lower() == 'f':
                        if c.isnumeric():
                            num = num*16 + int(c)
                        else:
                            match c.lower():
                                case 'a':
                                    num = num*16 + 10
                                case 'b':
                                    num = num*16 + 11
                                case 'c':
                                    num = num*16 + 12
                                case 'd':
                                    num = num*16 + 13
                                case 'e':
                                    num = num*16 + 14
                                case 'f':
                                    num = num*16 + 15
                                case _:
                                    raise Exception("Syntax error")
                        h += 1
                        c = sys.stdin.read(1)
                elif c == 'b' and num == 0 and count == 1:
                    h = 0
                    c = sys.stdin.read(1)
                    while c == '1' or c == '0':
                        if c.isnumeric():
                            num = num*2 + int(c)
                        h += 1
                        c = sys.stdin.read(1)

                tokens.insert(0, Token.NUM)
                stack.insert(0, num)
            case 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' |  \
                 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' |  \
                 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' |  \
                 'v' | 'x' | 'y' | 'z' |                    \
                 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' |  \
                 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' |  \
                 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' |  \
                 'V' | 'X' | 'Y' | 'Z' | '_':
                s = ""
                while c.isalpha() or c.isnumeric() or c == '_':
                    s += c
                    c = sys.stdin.read(1)
                
                if s == "exit":
                    break;
                elif s == "sqrt":
                    tokens.insert(0, Token.SQRT)
                elif s == "sin":
                    tokens.insert(0, Token.SIN)
                elif s == "cos":
                    tokens.insert(0, Token.COS)
                elif s == "exp":
                    tokens.insert(0, Token.EXP)
                elif s == "pi":
                    tokens.insert(0, Token.PI)
                elif s == "bin":
                    tokens.insert(0, Token.BIN)
                elif s ==  "hex":
                    tokens.insert(0, Token.HEX)
                elif s == "ans":
                    tokens.insert(0, Token.NUM)
                    stack.insert(0, ans)
                elif s == "align":
                    tokens.insert(0, Token.ALIGN)
                elif s != "":
                    if s not in symbol_table.keys():
                        symbol_table[s] = None
                    tokens.insert(0, Token.ID)
                    stack.insert(0, s)
                else:
                    print("Syntax Error\n")
                    tokens = []
                    c = sys.stdin.read(1)
            case _:
                break

def main():
        program()

main()
