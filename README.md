# QB-COM

> **QBasic/QuickBASIC 4.5 + QB64 Compiler in Rust**  
> เขียนโค้ด BASIC สมัยเก่า รันบนเครื่องสมัยใหม่

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

---

## 📦 ติดตั้ง (Installation)

### สิ่งที่ต้องมี
- [Rust](https://rustup.rs/) (เวอร์ชัน 1.70 ขึ้นไป)

### ติดตั้งแบบเร็ว

**Windows:**
```batch
setup.bat
```

**Linux/macOS:**
```bash
chmod +x setup.sh
./setup.sh
```

### ติดตั้งด้วยตนเอง
```bash
cargo build --release
```

---

## 🚀 วิธีใช้งาน (Usage)

### 1. รันโปรแกรม QBasic
```bash
# รันผ่าน cargo
cargo run --release -- run examples/hello.bas

# หรือถ้าติดตั้งแล้ว
qb run examples/hello.bas
```

### 2. คำสั่งทั้งหมด

| คำสั่ง | คำอธิบายสั้นๆ | ตัวอย่าง |
|--------|-------------|---------|
| `run <file>` | รันโปรแกรมทันที | `qb run hello.bas` |
| `build <file>` | คอมไพล์เป็น bytecode | `qb build hello.bas -o out.qbc` |
| `tokenize <file>` | ดู tokens ที่ lexer แยก | `qb tokenize hello.bas` |
| `parse <file>` | ดู AST (Abstract Syntax Tree) | `qb parse hello.bas` |
| `check <file>` | ตรวจสอบ error โดยไม่รัน | `qb check hello.bas` |
| `repl` | โหมด interactive (REPL) | `qb repl` |

---

## 📝 ตัวอย่างโค้ด (Examples)

### Hello World
```basic
PRINT "Hello, World!"
END
```

### ตัวแปรและชนิดข้อมูล
```basic
DIM name AS STRING
DIM age AS INTEGER
DIM pi AS SINGLE

name = "QB-COM"
age = 30
pi = 3.14159

PRINT "Name: "; name
PRINT "Age: "; age
PRINT "Pi: "; pi
END
```

### คำสั่งเงื่อนไข IF/THEN
```basic
DIM score AS INTEGER
score = 85

IF score >= 90 THEN
    PRINT "Grade A"
ELSEIF score >= 80 THEN
    PRINT "Grade B"
ELSEIF score >= 70 THEN
    PRINT "Grade C"
ELSE
    PRINT "Grade F"
END IF
END
```

### ลูป FOR/NEXT
```basic
DIM i AS INTEGER

FOR i = 1 TO 10
    PRINT "Count: "; i
NEXT i
END
```

### ลูป WHILE/WEND
```basic
DIM n AS INTEGER
n = 1

WHILE n <= 5
    PRINT n
    n = n + 1
WEND
END
```

### SELECT CASE
```basic
DIM choice AS INTEGER
choice = 2

SELECT CASE choice
    CASE 1
        PRINT "One"
    CASE 2
        PRINT "Two"
    CASE 3
        PRINT "Three"
    CASE ELSE
        PRINT "Other"
END SELECT
END
```

### Array (อาร์เรย์)
```basic
DIM arr(5) AS INTEGER

arr(0) = 10
arr(1) = 20
arr(2) = 30

PRINT arr(0); arr(1); arr(2)
END
```

### ฟังก์ชันทางคณิตศาสตร์
```basic
PRINT "ABS(-5) = "; ABS(-5)           ' ค่าสัมบูรณ์
PRINT "SQR(16) = "; SQR(16)           ' รากที่สอง
PRINT "INT(3.7) = "; INT(3.7)         ' ปัดเศษลง
PRINT "RND = "; RND                   ' สุ่มเลข 0-1
END
```

### ฟังก์ชันข้อความ
```basic
DIM text AS STRING
text = "Hello World"

PRINT LEFT$(text, 5)      ' ตัด 5 ตัวแรก: "Hello"
PRINT RIGHT$(text, 5)     ' ตัด 5 ตัวท้าย: "World"
PRINT MID$(text, 7, 5)    ' ตัดตั้งแต่ตัวที่ 7 จำนวน 5 ตัว: "World"
PRINT LEN(text)           ' ความยาว: 11
PRINT UCASE$(text)        ' ตัวพิมพ์ใหญ่: "HELLO WORLD"
PRINT LCASE$(text)        ' ตัวพิมพ์เล็ก: "hello world"
END
```

### TYPE (User-Defined Type)
```basic
TYPE Point
    x AS SINGLE
    y AS SINGLE
END TYPE

DIM p AS Point
p.x = 100
p.y = 200

PRINT "Point: ("; p.x; ", "; p.y; ")"
END
```

### CONST (ค่าคงที่)
```basic
CONST PI = 3.14159
CONST MAX_SIZE = 100

PRINT "PI = "; PI
PRINT "Max Size = "; MAX_SIZE
END
```

### File I/O (อ่าน/เขียนไฟล์)
```basic
' เขียนไฟล์
OPEN "data.txt" FOR OUTPUT AS #1
PRINT #1, "Hello File"
PRINT #1, "Line 2"
CLOSE #1

' อ่านไฟล์
DIM line AS STRING
OPEN "data.txt" FOR INPUT AS #2
LINE INPUT #2, line
PRINT "Read: "; line
CLOSE #2
END
```

### GOSUB/RETURN (ซับรูทีน)
```basic
PRINT "Start"
GOSUB MySub
PRINT "End"
END

MySub:
PRINT "  In subroutine"
RETURN
```

### DATA/READ (ข้อมูลในโปรแกรม)
```basic
DIM a, b, c AS INTEGER

READ a, b, c
PRINT a, b, c

DATA 10, 20, 30
END
```

---

## 🔧 QB64 Extensions (เฉพาะ QB-COM)

### ตัวแปร 64-bit
```basic
DIM big AS _INTEGER64
big = 9223372036854775807&&    ' ตัวเลขใหญ่สุด
PRINT big

DIM ul AS _UNSIGNED LONG
ul = 4000000000                 ' ค่าไม่ติดลบ
PRINT ul
END
```

### Metacommands
```basic
$CONSOLE              ' เปิดโหมด console
$INCLUDE:"file.bi"   ' รวมไฟล์อื่น
```

---

## 📋 ชนิดข้อมูลที่รองรับ (Data Types)

| ชนิด | ขนาด | คำอธิบาย | ตัวอย่าง |
|------|------|---------|---------|
| `INTEGER` | 16-bit | จำนวนเต็ม -32,768 ถึง 32,767 | `DIM x AS INTEGER` |
| `LONG` | 32-bit | จำนวนเต็มใหญ่ | `DIM x AS LONG` |
| `SINGLE` | 32-bit | ทศนิยม | `DIM x AS SINGLE` |
| `DOUBLE` | 64-bit | ทศนิยมความแม่นยำสูง | `DIM x AS DOUBLE` |
| `STRING` | ตัวแปร | ข้อความ | `DIM s AS STRING` |
| `_INTEGER64` | 64-bit | QB64: จำนวนเต็ม 64-bit | `DIM x AS _INTEGER64` |
| `_UNSIGNED LONG` | 32-bit | QB64: ไม่ติดลบ | `DIM x AS _UNSIGNED LONG` |

---

## 🏗️ สถาปัตยกรรมโปรเจค (Architecture)

```
QB-COM/
├── cli/           # คอมมานด์ไลน์อินเตอร์เฟซ
├── crates/
│   ├── core/      # ชนิดข้อมูลหลัก และ error handling
│   ├── lexer/     # แยกคำ (tokenizer)
│   ├── parser/    # วิเคราะห์โค้ด สร้าง AST
│   ├── semantic/  # ตรวจสอบ type และความถูกต้อง
│   ├── vm/        # Bytecode compiler + Virtual Machine
│   ├── codegen/   # Code generation (LLVM backend)
│   └── hal/       # Hardware abstraction (DOS emulation)
└── examples/      # ตัวอย่างโปรแกรม
```

---

## 🧪 รัน Test Suite

```bash
# รัน test ทั้งหมด
cargo test --release

# รันโปรแกรมตัวอย่าง
cargo run --release -- run examples/test_all.bas
```

---

## 📄 License

[MIT License](LICENSE) - ใช้งานได้ฟรี แก้ไขได้ แจกจ่ายได้

---

## 🔗 Repository

[https://github.com/thirawat27/QB-COM](https://github.com/thirawat27/QB-COM)
