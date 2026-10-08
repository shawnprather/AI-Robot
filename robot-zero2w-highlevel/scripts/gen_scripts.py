"""
Generates all gait scripts from one set of numbers.
Change SWING / LIFT / DELAY here and re-run: python gen_scripts.py
Writing angles by hand across 9 files is how sign mistakes happen.
"""

SWING = 20   # hip swing each way from neutral (degrees)
LIFT = 40    # foot lift from straight down (degrees)
DELAY = 200  # ms after each movement step

# Neutral angles (from the diagram)
HIP_NEUTRAL = {"QU": 45, "EU": 135, "ZU": 135, "CU": 45}
FOOT_NEUTRAL = {"QL": 0, "EL": 180, "ZL": 180, "CL": 0}

# Which way each servo turns for "forward" / "lift"
HIP_FWD = {"QU": -1, "EU": +1, "ZU": -1, "CU": +1}     # left hips decrease, right increase
FOOT_UP = {"QL": +1, "EL": -1, "ZL": -1, "CL": +1}     # diagonals match

# Diagonal pairs: leg letter -> (hip, foot)
PAIR_A = ["Q", "C"]  # front-left + rear-right
PAIR_B = ["E", "Z"]  # front-right + rear-left


def hip(leg, offset, direction):
    """Hip angle for a leg at `offset` stride units.
    direction +1 = leg walks forward pattern, -1 = backward pattern."""
    name = leg + "U"
    return name, HIP_NEUTRAL[name] + HIP_FWD[name] * direction * offset


def foot(leg, up):
    name = leg + "L"
    return name, FOOT_NEUTRAL[name] + FOOT_UP[name] * up


class Script:
    def __init__(self, title, notes):
        self.lines = [f"# {title}"] + [f"# {n}" for n in notes] + [""]

    def step(self, comment, moves):
        self.lines.append(f"# {comment}")
        for name, angle in moves:
            self.lines.append(f"SERVO {name} {angle}")
        self.lines.append(f"DELAY {DELAY}")
        self.lines.append("")

    def hold(self, ms, comment):
        self.lines.append(f"# {comment}")
        self.lines.append(f"DELAY {ms}")
        self.lines.append("")

    def save(self, filename):
        with open(filename, "w") as f:
            f.write("\n".join(self.lines))


def gait(filename, title, dirs, notes):
    """
    One full trot cycle that starts AND ends in the neutral stand,
    so it can repeat any number of times or chain into any other script.
    dirs: leg -> +1 (forward pattern) or -1 (backward pattern)
    """
    s = Script(title, notes + [
        "Starts and ends in neutral stand - run stand.txt first.",
        "Pair A = Q+C (front-left + rear-right), Pair B = E+Z (front-right + rear-left)",
    ])
    a_hips = lambda off: [hip(l, off, dirs[l]) for l in PAIR_A]
    b_hips = lambda off: [hip(l, off, dirs[l]) for l in PAIR_B]
    a_feet = lambda up: [foot(l, up) for l in PAIR_A]
    b_feet = lambda up: [foot(l, up) for l in PAIR_B]

    s.step("Lift pair A", a_feet(LIFT))
    s.step("A swings ahead in the air, planted B pushes back (body moves)",
           a_hips(+SWING) + b_hips(-SWING))
    s.step("Plant pair A", a_feet(0))
    s.step("Lift pair B", b_feet(LIFT))
    s.step("B swings ahead in the air (full stride), planted A pushes back",
           b_hips(+SWING) + a_hips(-SWING))
    s.step("Plant pair B", b_feet(0))
    s.step("Lift pair A", a_feet(LIFT))
    s.step("A swings to center in the air, planted B pushes back to center",
           a_hips(0) + b_hips(0))
    s.step("Plant pair A - back in neutral stand", a_feet(0))
    s.save(filename)


# ---------- stand ----------
s = Script("STAND - all servos neutral (X-shaped crab stance, feet straight down)", [])
s.step("Neutral pose",
       [(n, a) for n, a in HIP_NEUTRAL.items()] + [(n, a) for n, a in FOOT_NEUTRAL.items()])
s.save("stand.txt")

# ---------- gaits ----------
# Each leg either runs the "forward" pattern (+1) or "backward" pattern (-1).
# Mixing them per leg is what makes the robot walk, turn or crab.
F, B = +1, -1
gait("walk_forward.txt", "WALK FORWARD",
     {"Q": F, "E": F, "Z": F, "C": F}, ["All legs push backward -> body goes forward."])
gait("walk_backward.txt", "WALK BACKWARD",
     {"Q": B, "E": B, "Z": B, "C": B}, ["All legs push forward -> body goes backward."])
gait("turn_left.txt", "TURN LEFT (in place)",
     {"Q": B, "Z": B, "E": F, "C": F}, ["Tank-style: left legs walk backward, right legs walk forward."])
gait("turn_right.txt", "TURN RIGHT (in place)",
     {"Q": F, "Z": F, "E": B, "C": B}, ["Tank-style: left legs walk forward, right legs walk backward."])
gait("crab_right.txt", "CRAB WALK RIGHT (sideways)",
     {"Q": F, "C": F, "E": B, "Z": B},
     ["Pair A walks forward, pair B walks backward.",
      "Forward/back cancels out, sideways adds up -> body moves right (slight zigzag)."])
gait("crab_left.txt", "CRAB WALK LEFT (sideways)",
     {"Q": B, "C": B, "E": F, "Z": F},
     ["Pair A walks backward, pair B walks forward -> body moves left (slight zigzag)."])

# ---------- pushup ----------
s = Script("PUSHUP - all feet rotate up together, lowering the body, then back down", [
    "Starts and ends in neutral stand."])
s.step("Lower body (all feet toward horizontal)", [foot(l, 50) for l in "QEZC"])
s.hold(300, "Hold at the bottom")
s.step("Push back up", [foot(l, 0) for l in "QEZC"])
s.hold(300, "Hold at the top")
s.save("pushup.txt")

# ---------- sway ----------
s = Script("SWAY - lean left, center, lean right, center", [
    "Starts and ends in neutral stand.",
    "Lowering one side's feet tilts the body toward that side."])
s.step("Lean left (left feet up)", [foot("Q", 35), foot("Z", 35)])
s.step("Center", [foot("Q", 0), foot("Z", 0)])
s.step("Lean right (right feet up)", [foot("E", 35), foot("C", 35)])
s.step("Center", [foot("E", 0), foot("C", 0)])
s.save("sway.txt")

print("done")
