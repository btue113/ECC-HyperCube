# Soft-Decision Decoding on ECC-HyperCube

This note visualizes how the hypercube Reed–Muller decoder ([`CubeCode`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L22-L35) and [`ReedMuller`](file:///Users/buitue/projects/ecc-hypercube/src/reed_muller.rs#L7-L11)) works and how **soft-decision voting** and **level-by-level cancellation** operate on the hypercube.

---

## 1. Warm-Up: 3-Bit Codeword (`cw_dims = 2, p_dims = 1`)

A 2D square has $2^2 = 4$ vertices (`00`, `01`, `10`, `11`). Puncturing `00` leaves **3 non-zero vertices**:

```
       (10) --------- (11)   <-- Data bit (Level 0): vertex 11
        |              |
        |              |
       (00) --------- (01)
      unused         Parity bits (Level 1): vertices 10 and 01
```

* **Bit 0 (Data)**: vertex `11`
* **Bit 1 (Parity)**: vertex `10` — unit cube $\{10, 11\}$ must XOR to `0` $\implies c_{10} = c_{11}$
* **Bit 2 (Parity)**: vertex `01` — unit cube $\{01, 11\}$ must XOR to `0` $\implies c_{01} = c_{11}$

### The 3 Voting Cubes for `11`
Vertex `11` fixes both bit positions (`fixed_mask = 11`, `0` free bits). Its parallel cubes ([`HyperCube::parallel_cubes`](file:///Users/buitue/projects/ecc-hypercube/src/hypercube.rs#L56-L90)) are all non-zero assignments to those 2 fixed bits—each is a **0D sub-cube (1 vertex)**:

```
   fixed_value = 10               fixed_value = 11
      [Cube 3]                       [Cube 1]
        (10) ------------------------- (11)
         |                              |
         |                              |
         |                              |
        (00) ------------------------- (01)
     (excluded:                     [Cube 2]
   fixed_value = 00)             fixed_value = 01
```

1. **Cube 1 (`11`)**: $\{11\}$
2. **Cube 2 (`01`)**: $\{01\}$
3. **Cube 3 (`10`)**: $\{10\}$

### Hard vs. Soft Voting (`d = 1`, sent `[+1.0, +1.0, +1.0]`)

| Codeword Bit | Vertex | Sent | Received Soft Value | Hard Bit (`> 0?`) | Confidence (`\|y\|`) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Bit 0** | `11` | `+1.0` | **`+0.95`** | `1` | `0.95` *(Strong 1)* |
| **Bit 1** | `10` | `+1.0` | **`-0.05`** | `0` *(error)* | `0.05` *(Near-zero guess)* |
| **Bit 2** | `01` | `+1.0` | **`-0.08`** | `0` *(error)* | `0.08` *(Near-zero guess)* |

* **Hard Voting**: `1` gets 1 vote (`11`), `0` gets 2 votes (`10`, `01`) $\implies$ **Decodes `0` (Wrong)**
* **Soft Voting**:
  * Vote for `1`: `0.95`
  * Vote for `0`: `0.05 + 0.08 = 0.13`
  * **Decodes `1` (`0.95 > 0.13`, Correct)**

---

## 2. The 7-Bit Codeword (`cw_dims = 3, p_dims = 1`)

A 3D cube has $2^3 - 1 = 7$ non-zero vertices, ordered by weight:

```
         110 --------------- 111   <-- Level 0 (3 ones):  111          [1 Data bit]
        /|                  /|
       / |                 / |     <-- Level 1 (2 ones):  110, 101, 011 [3 Data bits]
     100 --------------- 101 |
      |  |                |  |     <-- Level 2 (1 one):   100, 010, 001 [3 Parity bits]
      |  010 -------------|- 011
      | /                 | /
      |/                  |/
     000 --------------- 001
  (unused)
```

* **Parity rule**: Every 4-vertex face (the unit cube of `100`, `010`, or `001`) has XOR sum = `0`.

### Voting for a Level 1 Data Bit (`110`)
[`CubeCode::decode`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L153) decodes lowest-weight data levels first (`Level 1` before `Level 0`).

Vertex `110` fixes the first 2 bits (`fixed_mask = 110`) and leaves the 3rd bit free (`_`). Its 3 voting parallel cubes are **3 disjoint horizontal edges (1D sub-cubes of 2 vertices each)**:

```
         110 =============== 111   <-- Edge 1 (11_): {110, 111}
        /|                  /|
       / |                 / |
     100 =============== 101 |     <-- Edge 3 (10_): {100, 101}
      |  |                |  |
      |  010 =============|= 011   <-- Edge 2 (01_): {010, 011}
      | /                 | /
      |/                  |/
     000 --------------- 001       <-- Edge 00_ excluded (000 unused)
```

### The "Weakest Link" (Min-Sum) Rule for Multi-Vertex Cubes
When a voting cube $C$ spans multiple vertices, its vote represents the XOR of those vertices:
1. **Vote Direction (Hard Parity)**: $\bigoplus_{v \in C} \text{hard}(v)$ (identical to [`CubeCode::vote`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L189-L233)).
2. **Vote Weight (Confidence)**: For the XOR sum to be right, *every* vertex in $C$ must be right. The reliability of the XOR sum is governed by its least reliable vertex:
   $$\text{Weight}(C) = \min_{v \in C} |y_v|$$

#### Walkthrough with 2 Bit Errors (All-Zero Codeword Sent as `-1.0`)
Suppose `110` flips strongly to **`+0.80`** and `010` barely flips to **`+0.05`**, while all other vertices are **`-0.90`**:

```
      +0.80 (1)          -0.90 (0)
         110 =============== 111       Edge 1 (11_): XOR = 1, min_conf = min(0.80, 0.90) = 0.80
        /|                  /|
       / |                 / |
     100 =============== 101 |         Edge 3 (10_): XOR = 0, min_conf = min(0.90, 0.90) = 0.90
  -0.90  |            -0.90  |
      |  010 =============|= 011       Edge 2 (01_): XOR = 1, min_conf = min(0.05, 0.90) = 0.05
      | / +0.05 (1)       | / -0.90
      |/                  |/
     000 --------------- 001
```

| Voting Cube | Vertices | Hard XOR | Soft Values | Cube Weight ($\min \|y_v\|$) | Weighted Vote |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **Edge 1 (`11_`)** | `{110, 111}` | `1 ^ 0 = 1` | `+0.80, -0.90` | $\min(0.80, 0.90) = \mathbf{0.80}$ | **`1` (weight `0.80`)** |
| **Edge 2 (`01_`)** | `{010, 011}` | `1 ^ 0 = 1` | `+0.05, -0.90` | $\min(0.05, 0.90) = \mathbf{0.05}$ | **`1` (weight `0.05`)** |
| **Edge 3 (`10_`)** | `{100, 101}` | `0 ^ 0 = 0` | `-0.90, -0.90` | $\min(0.90, 0.90) = \mathbf{0.90}$ | **`0` (weight `0.90`)** |

* **Hard Tally**: Two votes for `1` vs. one vote for `0` $\implies$ **Decodes `1` (Wrong)**
* **Soft Tally**:
  * Weight for `1`: $0.80 + 0.05 = \mathbf{0.85}$
  * Weight for `0`: $\mathbf{0.90}$
  * **Decodes `0` (`0.90 > 0.85`, Correct)**

---

## 3. Exact Probability Math: The $\tanh$ Rule vs. Min-Sum, and Combining Cubes

### Why the $\tanh$ Rule is Exact Inside a Parallel Cube (XOR)

Suppose a vertex $v$ has probability $p_v$ of being `0` and $1 - p_v$ of being `1`.
Define its **signed confidence ("soft spin")** $t_v \in [-1, +1]$ as:
$$t_v = P(v = 0) - P(v = 1) = 2p_v - 1 = \tanh(L_v / 2)$$
where $L_v = \ln \frac{p_v}{1 - p_v}$ is the Log-Likelihood Ratio (LLR).
* $t_v = +1.0$: 100% sure the bit is `0`
* $t_v = -1.0$: 100% sure the bit is `1`
* $t_v = 0.0$: 50/50 coin flip

When we XOR independent vertices $v_1 \oplus v_2 \oplus \dots \oplus v_k$ inside a parallel cube $C$, a classic identity of probability gives:
$$t_{\text{cube}}(C) = P\left(\bigoplus_{v \in C} v = 0\right) - P\left(\bigoplus_{v \in C} v = 1\right) = \prod_{v \in C} t_v = \prod_{v \in C} \tanh(L_v / 2)$$
Converting $t_{\text{cube}}(C)$ back to an LLR gives the **exact Box-Plus / $\tanh$ Rule**:
$$L(C) = 2 \operatorname{atanh}\left(\prod_{v \in C} \tanh(L_v / 2)\right)$$

#### Why $\tanh$ is More Accurate Than `min`:
* **One weak bit + one strong bit** (`0.80` and `0.99`):
  $0.80 \times 0.99 = \mathbf{0.792}$ — closely matches $\min(0.80, 0.99) = 0.80$! (This is why `min` is the famous fast approximation to the $\tanh$ rule).
* **Multiple moderately shaky bits** (`0.80` and `0.80`):
  $0.80 \times 0.80 = \mathbf{0.640}$, whereas `min` says `0.80`. The $\tanh$ product naturally captures the fact that if *several* vertices in a large sub-cube are somewhat noisy, their XOR is less reliable than any single one of them.

---

### Across Parallel Cubes: Summing $L(C)$ vs. Picking Only the Strongest Cube

Notice the fundamental difference between **inside** a cube and **across** parallel cubes:
* **Inside a parallel cube $C$**: Vertices are **XORed** ($v_1 \oplus v_2 = a$), so their reliabilities **multiply** in the $\tanh$ domain: $t(C) = \prod_{v \in C} \tanh(L_v / 2)$.
* **Across parallel cubes $C_1, C_2, \dots$**: Disjoint parallel cubes are **independent witnesses of the same bit $a$**. By Bayes' rule, independent log-likelihoods **add**:
  $$L_{\text{total}}(a) = \sum_{C \in \text{voters}(a)} L(C)$$

#### Why Summing $L(C)$ Beats Taking Only $\max_C |L(C)|$:
Suppose our 3 parallel cubes for `110` have LLRs:
* **Cube 1**: votes `1` with $L(C_1) = \mathbf{-0.85}$ (single strongest cube, hit by a large noise spike)
* **Cube 2**: votes `0` with $L(C_2) = \mathbf{+0.70}$ (strong `0`)
* **Cube 3**: votes `0` with $L(C_3) = \mathbf{+0.75}$ (strong `0`)

If we only picked the **single strongest cube**, Cube 1 (`-0.85`) would win and decode `1` (wrong), ignoring that **both** Cube 2 and Cube 3 strongly agreed on `0`.
By **summing** the cube LLRs:
$$L_{\text{total}} = -0.85 + 0.70 + 0.75 = \mathbf{+0.60} > 0 \implies \text{Decodes } 0 \text{ (Correct!)}$$
Meanwhile, if Cube 2 and Cube 3 were weak (`+0.05` and `+0.08`), the sum $-0.85 + 0.05 + 0.08 = -0.72$ still lets the single strong cube win automatically!

---

### Where Choosing the "Strongest Bits" Shines: Soft Cancellation & Reliability Ordering

Once we compute $L_{\text{total}}(a_u) = \sum_C L(C)$ for every bit $u$ in a level (e.g., `110`, `101`, `011` at Level 1):
* **Sign of $L_{\text{total}}(a_u)$** = decided coefficient (`0` or `1`)
* **Magnitude $|L_{\text{total}}(a_u)|$** = confidence in that decoded coefficient!

Ranking bits by $|L_{\text{total}}(a_u)|$ tells us **which decoded bits in the level are the strongest (most reliable)** and which are the weakest (near-tie votes):
1. **Soft Cancellation via the $\tanh$ Rule**:
   When cancelling a decided bit $a_u$ from vertex $v$, instead of treating $a_u$ as 100% certain ($\pm 1$), we can multiply $v$'s soft spin $t_v$ by $t(a_u) = \tanh(L_{\text{total}}(a_u) / 2)$:
   $$t_v^{\text{new}} = t_v^{\text{old}} \cdot \tanh(L_{\text{total}}(a_u) / 2)$$
   * If $a_u$ was a **strong bit** ($|L_{\text{total}}| \gg 1$, so $\tanh \approx \pm 1$), $t_v$ cleanly keeps its magnitude and flips sign if $a_u = 1$.
   * If $a_u$ was a **weak bit** ($L_{\text{total}} \approx 0$, so $\tanh \approx 0$), $t_v^{\text{new}}$ is automatically shrunk toward `0` so the uncertain cancellation cannot corrupt the next level!
2. **Branching on the Weakest Bit(s)**:
   If one bit in a level has very small $|L_{\text{total}}(a_u)|$, we can branch on both `0` and `1` for that weak bit while locking in all the strongest bits!

---

## 4. Level-by-Level Cancellation: Why & How It Works

### Why Do We Decode Level 1 Before Level 0, and Why Must We Cancel?

Think of the 4 data bits as generated by **4 hidden switches** (the coefficients $a_{111}$ at Level 0, and $a_{110}, a_{101}, a_{011}$ at Level 1).
Each switch $a_u$, when turned on (`1`), flips **every vertex $v \subseteq u$ (every subset of $u$)**:

* **$a_{111}$** flips all subsets of `111` (every vertex in the cube!).
* **$a_{110}$** flips subsets of `110`: `{110, 100, 010}` (back-top edge and its two vertical legs).
* **$a_{101}$** flips subsets of `101`: `{101, 100, 001}` (front face's corners).
* **$a_{011}$** flips subsets of `011`: `{011, 010, 001}` (right face's corners).

```
             [a_111 ^ a_110]                 [a_111]
                   110 ----------------------- 111
                   /|                          /|
                  / |                         / |
                 /  |                        /  |
[a_111^a_110^a_101] |       [a_111 ^ a_101] /   |
               100 ----------------------- 101  |
                |   |                       |   |
                |  010 ---------------------|-- 011
                |  / [a_111^a_110^a_011]    |  / [a_111 ^ a_011]
                | /                         | /
                |/                          |/
               000 ----------------------- 001
             (unused)              [a_111^a_101^a_011]
```

Look closely at the cube above:

1. **Why Level 1 (`a_110, a_101, a_011`) Can Vote Immediately**:
   When we XOR the two ends of any horizontal edge, **$a_{111}$ appears on both ends and cancels itself out ($a_{111} \oplus a_{111} = 0$)!**
   * Edge 1 (`110 ^ 111`): $(a_{111} \oplus a_{110}) \oplus a_{111} = \mathbf{a_{110}}$
   * Edge 2 (`010 ^ 011`): $(a_{111} \oplus a_{110} \oplus a_{011}) \oplus (a_{111} \oplus a_{011}) = \mathbf{a_{110}}$
   * Edge 3 (`100 ^ 101`): $(a_{111} \oplus a_{110} \oplus a_{101}) \oplus (a_{111} \oplus a_{101}) = \mathbf{a_{110}}$

2. **Why Level 0 (`a_111`) CANNOT Vote Until Level 1 is Cancelled**:
   Level 0 (`111`) votes using single vertices: `{111}`, `{011}`, `{101}`.
   Look at what sits on those 3 vertices in the diagram above:
   * Vertex `111` holds: $a_{111}$
   * Vertex `011` holds: $a_{111} \oplus \mathbf{a_{011}}$
   * Vertex `101` holds: $a_{111} \oplus \mathbf{a_{101}}$

   If $a_{101} = 1$, then vertex `101` holds $a_{111} \oplus 1$ — the **exact opposite** of $a_{111}$! Until we strip $a_{101}$ and $a_{011}$ off those vertices, `{011}` and `{101}` will vote against `{111}`!

---

### Visualizing a Full Cancellation Step on the Cube

Suppose our sent message has coefficients:
* **Level 0**: $a_{111} = \mathbf{1}$
* **Level 1**: $a_{110} = \mathbf{0}, \quad a_{101} = \mathbf{1}, \quad a_{011} = \mathbf{0}$

#### Before Cancellation (After Level 1 Votes)
Suppose there is no noise, and every `1` bit arrives as **`+0.90`** and every `0` bit arrives as **`-0.90`**.
Notice that because $a_{101} = 1$, vertices `101`, `100`, and `001` were flipped to `0` (`-0.90`) during encoding:

```
              +0.90 (1)                   +0.90 (1)
                 110 ----------------------- 111
                 /|                          /|
                / |                         / |
     -0.90 (0) /  |              -0.90 (0) /  |
             100 ----------------------- 101  |  <-- 101 has -0.90 (0) because a_101 = 1!
              |   |                       |   |
              |  010 ---------------------|-- 011
              |  / +0.90 (1)              |  / +0.90 (1)
              | /                         | /
              |/                          |/
             000 ----------------------- 001
                                       -0.90 (0)
```

If `111`'s three voters (`{111}`, `{011}`, `{101}`) tried to vote right now, `{111}` (`+0.90`) and `{011}` (`+0.90`) would vote `1`, while `{101}` (`-0.90`) would vote `0`!

#### Cancellation Step ([`cube_code.rs:171-178`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L171-L178))
1. Level 1 edges vote and discover: $a_{110} = 0, \mathbf{a_{101} = 1}, a_{011} = 0$.
2. To cancel $a_{101} = 1$, we flip the bit (`^= 1`) — which in soft terms means **negating the sign (`y = -y`)** — on `101`:

```
              +0.90 (1)                   +0.90 (1)   <-- Voter 1 for 111: +0.90 (1)
                 110 ----------------------- 111
                 /|                          /|
                / |                         / |
               /  |              +0.90 (1) /  |
             100 ----------------------- 101  |       <-- Voter 3 for 111: FLIPPED from -0.90 to +0.90 (1)!
              |   |                       |   |
              |  010 ---------------------|-- 011     <-- Voter 2 for 111: +0.90 (1)
              |  /                        |  /
              | /                         | /
              |/                          |/
             000 ----------------------- 001
```

3. Now all three Level 0 voters (`111`, `011`, `101`) hold **`+0.90 (1)`** and unanimously vote **$a_{111} = 1$**!
4. Finally, [`superset_xor(&mut coefficients)`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L179) turns the switches $(a_{111}=1, a_{110}=0, a_{101}=1, a_{011}=0)$ back into the 4 systematic data bits at `111, 110, 101, 011`: **`[1, 1, 0, 1]`**.

---

### What Happens to Soft Confidence During Cancellation?

There are two ways to handle the confidence $|y_v|$ when cancelling a decided coefficient $a_u = 1$ from a vertex $v$:

1. **Sign-Flip Only (Hard Cancellation, Simplest & Fastest)**:
   * Flipping a bit (`code[v] ^= 1`) simply negates the soft value (`y_v = -y_v`).
   * Because $|-y_v| = |y_v|$, **the vertex confidences never change** across levels!
   * You can keep the exact same bit-packed `code` array and cancellation loop from [`CubeCode::decode`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L171-L178), and just look up the original static channel confidences $|y_v|$ during `vote()`.

2. **Weakest-Link Soft Cancellation (Propagating Vote Margin)**:
   * What if Level 1's vote for $a_{101}$ was a squeaker (e.g., weight `0.51` for `1` vs `0.49` for `0`), so our confidence in $a_{101}$ is only $\text{margin}(a_{101}) = |0.51 - 0.49| = 0.02$?
   * Since the cancelled vertex value is $v_{\text{new}} = v_{\text{old}} \oplus a_{101}$, by the weakest-link rule its new reliability is:
     $$|y_v|_{\text{new}} = \min(|y_v|_{\text{old}}, \text{margin}(a_{101}))$$
   * That way, if a Level 1 coefficient was a coin-flip guess, any vertex that had $a_{101}$ cancelled out of it gets its confidence lowered to `0.02` so it cannot cast a high-weight bogus vote at Level 0!

---

## 5. The Vertex `000` Trick: Unlocking the Missing Parallel Cube in `ReedMuller`

In [`ReedMuller::encode`](file:///Users/buitue/projects/ecc-hypercube/src/reed_muller.rs#L40-L45), after [`CubeCode::encode`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L118-L133) computes the $2^m - 1$ non-zero vertices of the hypercube, an overall parity bit is appended as the $2^m$-th bit so the XOR over all $2^m$ bits is zero.

Previously, [`ReedMuller::decode`](file:///Users/buitue/projects/ecc-hypercube/src/reed_muller.rs#L47-L52) **discarded** that final bit (`&received[..received.len() - 1]`).

### Why That Final Bit *Is* Vertex `000`
Every parity vertex $p$ in the hypercube enforces that the XOR sum over its **unit cube** (all supersets of $p$) is `0`:
* Vertex `000` has **every vertex of the $m$-cube** as its superset.
* Therefore, the overall parity bit that makes the XOR over the entire $m$-cube zero is literally **vertex `000`**!

### What Placing the Final Bit at `000` Unlocks on the 3D Cube

```
         110 =============== 111   <-- Edge 1 (11_): {110, 111}
        /|                  /|
       / |                 / |
     100 =============== 101 |     <-- Edge 3 (10_): {100, 101}
      |  |                |  |
      |  010 =============|= 011   <-- Edge 2 (01_): {010, 011}
      | /                 | /
      |/                  |/
     000 =============== 001       <-- Edge 4 (00_): {000, 001}  [UNLOCKED!]
      ^
  Overall parity
  bit from RM!
```

1. **Completes the Partition of the Hypercube**:
   Without `000`, voting for `110` only had 3 edges (`11_`, `01_`, `10_`), leaving vertex `001` stranded without its partner (`000`). Placing the overall parity bit at `000` unlocks **Edge 4 (`00_`: `{000, 001}`)**, so all 8 vertices of the cube participate as **4 completely disjoint edges**.
2. **Zero Special Cases in `superset_xor`**:
   Because `000` is a subset of every vertex $u$, every switch $a_u$ flips `000`:
   * Vertex `000` holds: $a_{111} \oplus a_{110} \oplus a_{101} \oplus a_{011}$
   * Vertex `001` holds: $a_{111} \oplus a_{101} \oplus a_{011}$
   * Their XOR (`000 ^ 001`) is $\mathbf{a_{110}}$—and `superset_xor(&mut mask)` updates `mask[0]` automatically alongside all other vertices!
3. **Why Soft Decoding Benefits Enormously from the $2^{m-r}$-th Cube**:
   * In **hard voting**, moving from $3$ voters to $4$ voters can end in a `2 vs. 2` tie on 2-bit errors (detecting 2 errors, but not knowing which pair to trust).
   * In **soft voting**, continuous confidences never tie! When 2 bit errors hit 2 different edges:
     * With **3 edges** (no `000`), **2 out of 3 edges** are corrupted—the single clean edge is outnumbered 2-to-1.
     * With **4 edges** (with `000`), **2 edges are clean** and 2 edges are corrupted (and attenuated by their weak bits via the $\tanh$ rule), so the 2 clean edges outvote the 2 noisy edges, cutting the block error rate in half!

---

## 6. Prototype Implementation & AWGN Results

We implemented the prototype in [`CubeCode::decode_soft`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L258-L353) and [`ReedMuller::decode_soft`](file:///Users/buitue/projects/ecc-hypercube/src/reed_muller.rs#L54-L60):

1. **Soft Spin Precomputation**:
   Each vertex's received value $y_v$ (`> 0` favors `1`, `< 0` favors `0`) is converted once into its soft spin $t_v = -\tanh(y_v / 2) \in (-1, +1)$.
2. **Exact Box-Plus ($\tanh$/$\operatorname{atanh}$) Voting ([`CubeCode::vote_soft`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L326-L353))**:
   Each parallel cube multiplies the spins of its vertices ($t(C) = \prod_{v \in C} t_v$) and converts back to an LLR via $-2 \operatorname{atanh}(t(C))$, summing across all parallel cubes.
3. **Full-Cube Cancellation & Voting**:
   Decided coefficients at each level are cancelled out of all lower levels via [`superset_xor`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L308-L313) (`spins[v] = -spins[v]`), allowing higher-weight data bits (such as `111` at Level 0) to gather votes from all $2^{\text{weight}} - 1$ non-zero parallel sub-cubes instead of only $2^{\text{p\_dims}+1} - 1$.
4. **Unlocking Vertex `000` in [`ReedMuller::decode_soft`](file:///Users/buitue/projects/ecc-hypercube/src/reed_muller.rs#L54-L60)**:
   Instead of discarding the overall parity bit at the end of the `ReedMuller` codeword, `ReedMuller::decode_soft` supplies it as vertex `0` (`zero_vertex_llr`). This unlocks the $2^{m-r}$-th disjoint parallel sub-cube (`base == 0`), so every data bit votes over $2^{\text{weight}}$ completely disjoint sub-cubes partitioning the entire $2^m$ hypercube.

### Verification & AWGN Channel Benchmark ([`tests/roundtrip.rs`](file:///Users/buitue/projects/ecc-hypercube/tests/roundtrip.rs#L55-L103))

* **Walkthrough Unit Tests ([`soft_decoding_3bit_and_7bit_examples`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L502-L523))**:
  * **3-bit code (`cw_dims = 2, p_dims = 1`)** on `[+0.95, -0.05, -0.08]`: hard decoder outputs `[false]` (wrong), `decode_soft` outputs `[true]` (correct).
  * **7-bit code (`cw_dims = 3, p_dims = 1`)** with 2 bit errors (`+0.80` on `110` and `+0.05` on `010`): hard decoder fails, `decode_soft` recovers `[false; 4]`.

* **Simulated AWGN Channel Block Error Rates**:

| Code | Block/Msg $(n, k)$ | Noise $\sigma$ | Hard Decoder Errors | Soft Decoder (Punctured, no Vertex `0`) | Soft Decoder (with Vertex `0` + Full-Cube Voters) |
| :--- | :---: | :---: | :---: | :---: | :---: |
| `RM(0, 2)` | $(4, 1)$ | `0.90` | `43 / 1000` | `20 / 1000` | **`15 / 1000` (2.9× fewer)** |
| `RM(1, 3)` | $(8, 4)$ | `0.70` | `101 / 1000` | `60 / 1000` | **`30 / 1000` (3.4× fewer)** |
| `RM(1, 4)` | $(16, 5)$ | `0.75` | `31 / 1000` | `12 / 1000` | **`6 / 1000` (5.2× fewer)** |
| `RM(2, 5)` | $(32, 16)$ | `0.65` | `59 / 500` | `31 / 500` | **`19 / 500` (3.1× fewer)** |

---

## 7. Why Single-Pass Level Voting Falls Behind at Larger $n$, and Two List-Decoding Worktrees

When we scale up from $(8, 4)$ and $(16, 5)$ to $(32, 16)$, $(64, 42)$, and $(128, 64)$ at $E_b / N_0 = 3\text{ dB}$, the single-pass level voter falls increasingly behind a list decoder:

1. **Many Uncoordinated Parallel Decisions in the First Level**:
   * At $(32, 16)$ ($RM(2, 5)$), the lowest-weight data level has $\binom{5}{3} = \mathbf{10}$ coefficients.
   * At $(64, 42)$ ($RM(3, 6)$), the first level has $\binom{6}{3} = \mathbf{20}$ coefficients.
   * At $(128, 64)$ ($RM(3, 7)$), the first level has $\binom{7}{4} = \mathbf{35}$ coefficients!
   * Even if each individual coefficient vote is $97\%$ reliable, making 35 independent parallel decisions without checking parity constraints means all 35 are simultaneously right only $0.97^{35} \approx 34\%$ of the time.
2. **Large Voting Sub-Cubes ($2^r$ Vertices)**:
   * At $r = 3$, each parallel voter is an 8-vertex sub-cube ($2^3 = 8$). Multiplying 8 soft spins ($\prod_{i=1}^8 \tanh(L_i / 2)$) attenuates the cube reliability toward zero whenever multiple vertices in the same cube have moderate noise.
3. **No Parity Verification or Path Recourse**:
   * In the one-pass decoder, once a single coefficient at the bottom level is misdecided, cancelling it flips $2^{m-r}$ vertices across the cube and corrupts all subsequent levels, and the parity vertices (`self.data_len..`) are never checked at the end.

---

## 8. Visualizing Approach B: The Recursive Hypercube (SCL) Decoder

> **Core Intuition**:
> * The original algorithm decodes **Level by Level** (horizontally across weights).
> * The Recursive Hypercube (SCL) algorithm slices the cube in half **Dimension by Dimension**—folding the hypercube like origami until only single vertices remain!

### 1. Folding a 1D Edge in Half (`superset_xor` in Reverse)

Recall how the hidden switches ($a_0, a_1$) produce the codeword bits ($c_0, c_1$) on a single 1D edge:

```
      c_0 = a_0 ^ a_1               c_1 = a_1
          (0) ------------------------- (1)
         lower                         upper
```

* The **upper** vertex `(1)` holds just $a_1$.
* The **lower** vertex `(0)` holds $a_0 \oplus a_1$ (because `0` is a subset of `1`, so $a_1$ spills into `(0)`).

Given noisy soft measurements $L_0$ and $L_1$, how do we recover $a_0$ and $a_1$?

#### Step 1: Fold Upper onto Lower to Expose `a_0` (The XOR / Weakest-Link Step)
If we XOR the two ends of the edge together, $a_1$ cancels out:
$$c_0 \oplus c_1 = (a_0 \oplus a_1) \oplus a_1 = \mathbf{a_0}$$
By our Weakest-Link (Min-Sum) rule, the soft reliability of $c_0 \oplus c_1$ is:
$$L_{\text{left}} = \text{sign}(L_0)\text{sign}(L_1) \min(|L_0|, |L_1|)$$
This gives us a direct soft measurement of **$a_0$**!

#### Step 2: Once `a_0` is Known, Combine Both Ends to Boost `a_1` (The Copy / Sum Step)
Once we decide the lower bit $u_0 = a_0$:
* Vertex `(1)` is already a direct measurement of **$a_1$** (with LLR $L_1$).
* Vertex `(0)` holds $a_1 \oplus u_0$. Since we **now know $u_0$**, we strip $u_0$ off `(0)` (flipping $L_0$'s sign if $u_0 = 1$), turning `(0)` into a **second independent witness of $a_1$**!
* Two independent witnesses of the same bit **add their LLRs together**:
  $$L_{\text{right}} = \begin{cases} L_1 + L_0 & \text{if } u_0 = 0 \\ L_1 - L_0 & \text{if } u_0 = 1 \end{cases}$$

Notice the fundamental asymmetry:
* **Step 1 (Lower half / `0`)** has to **XOR** two noisy vertices, so its confidence gets **weaker** ($\min(|L_0|, |L_1|)$).
* **Step 2 (Upper half / `1`)** gets to **ADD** two vertices together after stripping $u_0$, so its confidence gets **twice as strong** ($L_1 \pm L_0$)!

---

### 2. Slicing the 3D Cube (`RM(1, 3)` — 8 Vertices)

Now watch what happens when we apply this to the 3D cube (`000` to `111`).
Instead of looking at individual edges, we **slice the entire 3D cube in half**, separating the **Left 2D Face (`0__`)** from the **Right 2D Face (`1__`)**:

```
       LEFT 2D FACE (0__)                  RIGHT 2D FACE (1__)
      (lower half: bit 2 = 0)             (upper half: bit 2 = 1)

         010 --------- 011                   110 --------- 111
          |             |      <== fold ==>   |             |
          |             |     4 parallel      |             |
         000 --------- 001      bridges      100 --------- 101
```

There are **4 parallel bridges** connecting the Left Face (`0__`) to the Right Face (`1__`):
* `000 <===> 100`
* `001 <===> 101`
* `010 <===> 110`
* `011 <===> 111`

#### Phase 1: Fold the Right Face onto the Left Face (`0__`)
We apply Step 1 (`min_sum`) across all 4 bridges at once, folding the 3D cube into the **Left 2D Square (`0__`)**.
Then we fold that 2D square in half into a **1D Edge (`00_`)**, and fold the 1D edge into **single vertices**:
1. First leaf reached: **Vertex `000`** (weight 0 $\rightarrow$ **Parity / Frozen to `0`**!)
2. Second leaf: **Vertex `001`** (weight 1 $\rightarrow$ **Parity / Frozen to `0`**!)
3. Third leaf: **Vertex `010`** (weight 1 $\rightarrow$ **Parity / Frozen to `0`**!)
4. Fourth leaf: **Vertex `011`** (weight 2 $\rightarrow$ **First Data Bit!**)

Look at what happened before we ever had to make a single data decision at `011`:
* Vertices `000`, `001`, and `010` are **Parity vertices**—we **already know** their hidden switches are `0`!
* We don't have to guess them: we lock in `0` for `000`, `001`, and `010`, and each time we lock one in, Step 2 (`L_upper + L_lower`) **adds** their soft measurements into the remaining vertices.
* By the time we reach our very first data vertex (`011`), its LLR has absorbed the combined soft evidence of the entire Left Face!

#### Phase 2: Use the Solved Left Face (`0__`) to Boost the Right Face (`1__`)
Once the Left Face (`000, 001, 010, 011`) is decided, we strip its bits off the 4 bridges and **add** the Left Face's LLRs onto the Right Face (`1__`).
Now the Right Face (`100, 101, 110, 111`) has **doubled signal strength**, and we recursively fold and decode it the exact same way (`100` is a parity bit frozen to `0`, while `101`, `110`, `111` are data bits)!

---

### 3. How List Decoding ($L$ Paths) Uses Parity Vertices as "Police Checkpoints"

What if, when we reach data vertex `011`, noise makes its soft score $L_{011}$ a near-tie (say **`+0.05`**)?
* A single-pass decoder must guess `0` or `1` immediately. If it guesses wrong, cancelling `011` across the bridge corrupts the entire Right Face.
* **List Decoding (SCL)** instead forks the path into **two parallel universes** (`a_011 = 0` and `a_011 = 1`) and keeps both alive:

```
                        [Start: 1 Path (penalty = 0)]
                                    |
                   (000, 001, 010 are Parity -> locked to 0)
                                    |
                         Reach Data Vertex 011
                          (LLR = +0.05, shaky!)
                            /               \
               Branch A: a_011 = 0       Branch B: a_011 = 1
                (penalty: +0.00)          (penalty: +0.05)
                       |                         |
             [Reach Parity 100]        [Reach Parity 100]
             LLR says 0 (+2.40)        LLR says 1 (-2.40) <-- Contradicts Parity = 0!
             (penalty: +0.00)          (penalty: +2.45!)  <-- Heavy penalty!
```

Look at what happens at the very next vertex (**`100`**, which is a **Parity vertex** whose switch *must* be `0`):
1. In **Branch A** (`a_011 = 0`), when we step onto parity vertex `100`, its combined LLR strongly favors `0` (`+2.40`) $\rightarrow$ **`0` penalty!**
2. In **Branch B** (`a_011 = 1`), the wrong choice at `011` flipped the bridge into `100`, so `100`'s LLR screams `-2.40` (trying to be `1`, which is illegal for a parity vertex!) $\rightarrow$ **Slapped with a `+2.40` penalty!**

**Every parity vertex acts as a soft "police checkpoint" interspersed between the data vertices!**
Whenever a candidate path makes a wrong turn at a shaky data vertex, subsequent parity checkpoints catch the contradiction and penalize that branch. By keeping the top $L$ paths (e.g., $L = 8$ or $32$) with the lowest total penalty, the true path survives and wins at the end.

---

### 4. Side-by-Side Summary: Level-by-Level Voter vs. Recursive Hypercube (SCL)

| Feature | Level-by-Level Hypercube Voter | Recursive Hypercube (SCL) |
| :--- | :--- | :--- |
| **Order of visiting vertices** | By **weight** (`011, 101, 110` all at once, then `111`). | By **cube index** (`000, 001, 010, 011, 100, 101, 110, 111`). |
| **Does `101` benefit from knowing `011`?** | **No.** `011` and `101` vote simultaneously in the same level without talking to each other. | **Yes!** `011` is decided and cancelled *before* `101` is ever visited. |
| **Role of parity vertices (`000, 001, 010, 100`)** | Used only inside edge XORs; never checked for consistency after voting. | Visited in order as **frozen (`0`) checkpoints** that immediately penalize wrong branches. |
| **Recovering codeword bits at the end** | Requires a final `superset_xor` pass to convert switches $a$ into codeword bits $c$. | Unwinding the recursion (`u_lower ^= u_upper`) **is** `superset_xor`, so the systematic codeword bits pop out directly in-place! |

---

### Head-to-Head Comparison of the Two Worktrees at $E_b / N_0 = 3\text{ dB}$ (2,000 Words)

We created two git worktrees to compare two list-decoding architectures directly on `ReedMuller`:

1. **Worktree 1 (`soft-cube-list`) — [src/cube_code.rs](file:///Users/buitue/projects/ecc-hypercube-cube-list/src/cube_code.rs#L252-L455)**:
   * Keeps the parallel-cube voter (`vote_soft`), branches on the $K$ weakest coefficient votes in each level (pruning to `list_size` paths), and ranks the final candidates by exact Maximum-Likelihood (ML) codeword correlation across all $2^m$ vertices (including all parity vertices and vertex `0`).
2. **Worktree 2 (`soft-scl`) — [src/cube_code.rs](file:///Users/buitue/projects/ecc-hypercube-scl/src/cube_code.rs#L252-L444)**:
   * Exploits the fact that [`superset_xor`](file:///Users/buitue/projects/ecc-hypercube-scl/src/cube_code.rs#L261-L272) is the exact Plotkin / Polar butterfly transform $G_N = \begin{bmatrix} 1 & 1 \\ 0 & 1 \end{bmatrix}^{\otimes m}$.
   * Recursively splits the $m$-dimensional hypercube into lower and upper $(m-1)$-dimensional sub-cubes (`scl_step`):
     * **Lower half (`bit dim-1 = 0`)**: combines LLRs via `box_plus(L_lower, L_upper)`.
     * **Upper half (`bit dim-1 = 1`)**: cancels the decided lower-half partial sums immediately via `L_upper ± L_lower`.
     * **Leaf (`dim == 0`)**: if `vertex.count_ones() < p_dims + 1` (a parity vertex), the coefficient is frozen to `0` and immediately penalizes paths that disagree; if it is a data vertex, forks each path into `0` and `1` and retains the best `list_size` paths.

| Code $(n, k)$ | RM hard (2013) | RM soft ($L=1$, 1-pass) | **Worktree 1: `soft-cube-list` ($L=8 / 32$)** | **Worktree 2: `soft-scl` ($L=8 / 32$)** | Polar, list of 8 | Polar + CRC, list of 8 | Limit for any code |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **`(8, 4)`** | `0.113` | `0.033` | **`0.026 / 0.026`** | **`0.026 / 0.026`** | `0.027` | — | — |
| **`(16, 5)`** | `0.114` | `0.048` | **`0.015 / 0.015`** | **`0.015 / 0.015`** | `0.015` | — | — |
| **`(32, 16)`** | `0.203` | `0.112` | `0.044 / 0.029` | **`0.011 / 0.011`** | `0.033` | `0.043` | `0.010` |
| **`(32, 26)`** | `0.320` | `0.202` | `0.141 / 0.118` | **`0.095 / 0.095`** | `0.082` | — | `0.094` |
| **`(64, 42)`** | `0.414` | `0.305` | `0.210 / 0.165` | **`0.020 / 0.018`** | `0.038` | `0.056` | `0.007` |
| **`(128, 64)`** | `0.739` | `0.561` | `0.404 / 0.344` | **`0.006 / 0.001`** | `0.026` | `0.006` | `≈0` |

### Key Takeaways from the Comparison
* **At short block lengths ($(8, 4)$ and $(16, 5)$)**: `soft-cube-list` and `soft-scl` achieve **identical** word error rates (`0.026` and `0.015`), because branching on the weakest level votes and scoring full codewords against all parity vertices explores the full candidate space.
* **At medium/larger block lengths ($(32, 16)$ through $(128, 64)$)**: `soft-scl` pulls decisively ahead (`0.011` vs `0.029` at $n=32$; `0.018` vs `0.165` at $n=64$; `0.001` vs `0.344` at $n=128$) because it conditions **every single vertex decision** on all prior decisions (via immediate butterfly cancellation) and enforces parity-vertex (`frozen = 0`) penalties continuously during the recursion rather than waiting until an entire level of 20–35 vertices has voted in parallel.

---

## 9. 10× Speedup on `soft-scl` (Merged to `main`)

In the initial recursive SCL implementation, `L = 8` was ~30–40× slower than hard decoding (`26 µs` at $n = 32$, `67 µs` at $n = 64$, `134 µs` at $n = 128$) due to per-node heap allocations, cloning the full `coefficients` vector at every data leaf, and transcendental `tanh`/`atanh`/`exp`/`ln_1p` calls.

We optimized [`SclDecoder`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L305-L460) with three structural changes:

1. **Eliminated the `coefficients` Vector & Final `superset_xor` Pass**:
   Because the recursive butterfly combine (`u_lower[i] ^= u_upper[i]`) in [`SclDecoder::step`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L441-L449) already computes the exact `superset_xor` transform in-place as the recursion unwinds, the root `sums` buffer of the winning path **already holds the final systematic codeword vertices** at the end of `step(self.dims, 0)`.
2. **Flat Pre-Allocated Workspace + In-Place First Child (No Path Cloning)**:
   * All $L$ paths live in a single pre-allocated contiguous buffer (`llrs` of size $L \times 2n$ and `sums` of size $L \times n$) with zero heap allocations during recursion.
   * At each data leaf (`dim == 0`), the first surviving child of each parent stays **in-place** in its parent's slot (`0` bytes copied); only second children (actual forks) copy into an unused slot via `copy_within` (and only copy `0..v` in `sums`).
   * Candidate pruning uses $O(L)$ partial selection (`select_nth_unstable_by`) instead of full sorting.
3. **Min-Sum Box-Plus (`min_sum`) + Piecewise-Linear Penalty**:
   * Replaced `2 * atanh(tanh(l1/2) * tanh(l2/2))` with [`min_sum(l1, l2)`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L453-L460) (`sign(l1) * sign(l2) * min(|l1|, |l2|)`).
   * Replaced `ln(1 + exp(-l))` with the piecewise-linear min-sum penalty (`if l < 0.0 { metric -= l }`).

### Speed & WER After Optimization

#### 1. Time per Word (Microseconds — `cargo bench --bench codec` vs. Realistic AWGN)

*Note: `cargo bench --bench codec` times a single lightly noisy test word (`12.84 µs` at $L=8$, `46.38 µs` at $L=32$ for $n=128$). On simulated AWGN channel noise at $3\text{ dB}$ (including the clean-word fast path), average decode times are slightly higher (`~19 µs` at $L=8$, `~55 µs` at $L=32$ for $n=128$) due to more frequent path forks and fewer trivial comparisons.*

| $n$ | Hard (`codec` bench) | **SC ($L=1$): Before $\to$ After** | **SCL ($L=8$): Before $\to$ After (AWGN avg)** | **SCL ($L=32$): Before $\to$ After (AWGN avg)** |
| :---: | :---: | :---: | :---: | :---: |
| **`32`** | `0.44 µs` | `7 µs` $\to$ **`0.73 µs`** | `26 µs` $\to$ **`2.37 µs`** (`~3.5 µs` AWGN) | `80 µs` $\to$ **`7.97 µs`** |
| **`64`** | `0.99 µs` | `16 µs` $\to$ **`1.62 µs`** | `67 µs` $\to$ **`6.37 µs`** | `230 µs` $\to$ **`24.12 µs`** |
| **`128`** | `2.30 µs` | `31 µs` $\to$ **`2.93 µs`** | `134 µs` $\to$ **`12.84 µs`** (`~19 µs` AWGN) | `460 µs` $\to$ **`46.38 µs`** (`~55 µs` AWGN) |

#### 2. Word Error Rate at $E_b / N_0 = 3\text{ dB}$ (2,000 Words)

| Code $(n, k)$ | Exact `tanh`/`atanh` ($L=8 / 32$) | **Fast Min-Sum ($L=8 / 32$)** |
| :--- | :---: | :---: |
| **`(8, 4)`** | `0.026 / 0.026` | **`0.026 / 0.026`** |
| **`(16, 5)`** | `0.015 / 0.015` | **`0.015 / 0.015`** |
| **`(32, 16)`** | `0.011 / 0.011` | **`0.011 / 0.011`** |
| **`(32, 26)`** | `0.095 / 0.095` | **`0.095 / 0.095`** |
| **`(64, 42)`** | `0.020 / 0.018` | **`0.021 / 0.018`** |
| **`(128, 64)`** | `0.006 / 0.001` | **`0.006 / 0.001`** |

---

## 10. Honest Positioning: Where `ecc-hypercube` Stands vs. 5G Polar & Modern ECC

`ecc-hypercube` is a fast, dependency-free software Reed–Muller and hybrid RM+CRC codec:
* **Algorithmic pedigree**:
  * **Soft decoder**: **Dumer–Shabunov recursive list decoding (2006)** combined with **Tal–Vardy-style in-place list management (2015)** and **Balatsoukas-Stimming et al. min-sum LLR metrics (2015)**.
  * **CRC-Aided RM (`with_crc`)**: Unfreezes the $c$ highest-reliability weight-$(m-r-1)$ parity vertices (ranked via **He et al. 2017 $\beta$-expansion**, $\beta = 2^{1/4}$, without a lookup table) and places the $c$-bit CRC on the **highest-index coefficient positions** (`u_crc_vertices`) while the weight-$(m-r-1)$ vertices carry payload—eliminating all weight-8 impostors ($A_8 = 0$ at $n = 128$, CRC-6/8) under CA-SCL and preserving the 2013 bit-packed majority-logic hard decoder (`decode()`).
* **Original contributions**: The **unit-cube / superset formulation** (`bit_to_cube`, `superset_xor` involution) and a unified implementation where clean-word syndrome/CRC checking, SWAR bit-packed multi-level majority voting, and in-place CA-SCL all operate on the exact same butterfly representation.
* **Best niche**: Short blocks ($n \le 128$) on a CPU at RM payload dimensions ($k = \sum_{i=0}^r \binom{m}{i}$), either without CRC (`ReedMuller::new`) or with a 6- to 8-bit CRC (`ReedMuller::with_crc`).

---

### 1. Head-to-Head vs. 5G Polar: Four Benchmark Modes ([`benches/polar_5g.rs`](file:///Users/buitue/projects/ecc-hypercube/benches/polar_5g.rs))

Comparing Reed–Muller against Polar codes requires separating **code distance ($d_{\min}$)**, **channel polarization**, and **how the CRC is accommodated**. [`benches/polar_5g.rs`](file:///Users/buitue/projects/ecc-hypercube/benches/polar_5g.rs) runs all four comparisons on AWGN with BPSK using the same min-sum SCL decoder and 5G reliability sequence $Q_0^{N_{\max}-1}$:

1. **No CRC on either side (`noCRC`)**: Pure RM vs. bare Polar at equal $(n, K)$. **RM wins by `4×–30×` at $n = 64\text{--}128$** (and ties at $n = 32$), while bare Polar wins at $n = 256$.
2. **In-band CRC stealing RM data bits (`sameCRC` / Way 1)**: Both codes use fixed dimension $K = k_{\text{RM}}$ and steal $c$ bits from the payload ($A = K - c$). **Polar + CRC wins at all $n \ge 64$ (`2×` at $n = 64$, `4×–31×` at $n = 128$, `400×–1000×` at $n = 256$)** because RM pays the $10\log_{10}(k/(k-c))$ dB rate tax while keeping its poorly polarized early weight-$(m-r)$ channels.
3. **Out-of-band CRC unfreezing weight-$(m-r-1)$ parity vertices (`rmcrc` / Way 2 — `ReedMuller::with_crc`)**: Both codes carry the full $k$-bit RM payload and unfreeze $c$ extra channels ($K = k + c$), with the CRC placed on the last $c$ coefficient indices:
   * **At $n = 32$, $n = 64$, and `(128, 64+11)`**: `ReedMuller::with_crc` and 5G Polar share the **same information set** and **tie** across all list sizes (`1.7e-2` vs. `1.8e-2` at $n=32$; `9.9e-4` vs. `9.5e-4` at $n=64$, CRC-6; `1.4e-4` vs. `1.6e-4` at `(128, 64+11)`).
   * **At `(128, 64+6)`, $L = 32$**: **`ReedMuller::with_crc(3, 7, 6)` beats 5G Polar by `5–7×` (`~5e-5` vs. `~3e-4`)** and beats plain RM by **`~3×`** (`~5e-5` vs. `1.5e-4`), because keeping all 64 weight-$\ge 4$ vertices and only 6 weight-3 vertices allows the 6-bit CRC to eliminate every weight-8 codeword ($A_8 = 0$), whereas 5G Polar swaps `{15, 23}` for 2 extra weight-3 vertices (`816` weight-8 codewords).
   * **At `(128, 64+6/8)`, $L = 8$**: **5G Polar wins by `~2×`** (`3.7e-4`–`4.4e-4` vs. `8.8e-4`–`8.9e-4`), because freezing `{15, 23}` prevents early list eviction when the list is small ($L = 8$).
4. **Pure RM (no CRC) vs. Full 5G NR Uplink Polar (with CRC/PC) (`5g`)**: At equal user payload $A = K_{\text{RM}}$, pure RM beats 5G Polar on total word errors for $n \le 128$ only when RM skips the CRC (all errors silent) and 5G Polar pays the rate tax for one.

#### A. No CRC on Either Side (`noCRC`: Bare RM vs. Bare Polar at Equal $(n, K)$)

| Code $(n, K)$ | $E_b / N_0$ | $L = 8$: `RM vs. Polar` | $L = 32$: `RM vs. Polar` | Winner & Margin ($L = 8 \text{ / } L = 32$) |
| :--- | :---: | :---: | :---: | :--- |
| **`(32, 16)`** | `4.0 dB` | `1.87e-3` vs. `1.87e-3` | `1.82e-3` vs. `1.81e-3` | **Exact tie** *(identical 16 generator rows)* |
| **`(64, 42)`** | `4.0 dB` | **`1.40e-3`** vs. `7.54e-3` | **`1.34e-3`** vs. `7.67e-3` | **RM wins `5.4× / 5.7×`** |
| **`(128, 29)`** | `3.0 dB` | **`1.29e-3`** vs. `5.88e-3` | **`3.82e-4`** vs. `5.92e-3` | **RM wins `4.6× / 15.5×`** |
| **`(128, 64)`** | `3.5 dB` | **`8.50e-4`** vs. `3.28e-3` | **`1.12e-4`** vs. `3.33e-3` | **RM wins `3.9× / 29.7×`** |
| **`(128, 99)`** | `4.0 dB` | **`3.70e-3`** vs. `1.50e-2` | **`2.27e-3`** vs. `1.51e-2` | **RM wins `4.1× / 6.7×`** |
| **`(256, 93)`** | `3.0 dB` | `1.34e-2` vs. **`1.38e-3`** | `1.96e-3` vs. **`1.42e-3`** | **Polar wins `9.7× / 1.4×`** |

* **Why Bare Polar stagnates from $L=8$ to $L=32$ while RM improves**: At `(128, 64)`, $RM(3, 7)$ chooses all 64 rows of weight $\ge 4$, giving minimum distance $d_{\min} = 2^4 = 16$. Bare Polar swaps several early weight-4 rows for late weight-3 rows, dropping $d_{\min}$ to $2^3 = 8$. Without a CRC, the transmitted codeword in bare Polar is already in the list at $L=8$, but loses the final metric comparison to a nearby weight-8 codeword—so $L=32$ gives zero improvement (`3.28e-3` $\to$ `3.33e-3`).

#### B. Same CRC on Both Sides (`sameCRC`: RM + CRC vs. Polar + CRC at Equal $(n, \text{payload} + \text{CRC})$)

| Code $(n, \text{payload}+\text{CRC})$ | $E_b / N_0$ | $L = 8$: `RM vs. Polar` | $L = 32$: `RM vs. Polar` | Winner & Margin ($L = 8 \text{ / } L = 32$) |
| :--- | :---: | :---: | :---: | :--- |
| **`(32, 10+6)`** | `4.0 dB` | `8.09e-3` vs. `8.09e-3` | `3.14e-3` vs. `3.15e-3` | **Exact tie** *(identical 16 generator rows)* |
| **`(64, 31+11)`** | `4.0 dB` | `1.13e-2` vs. **`5.39e-3`** | `2.09e-3` vs. **`1.19e-3`** | **Polar wins `2.1× / 1.8×`** |
| **`(128, 18+11)`** | `3.0 dB` | `8.29e-2` vs. **`1.90e-2`** | `2.35e-2` vs. **`5.48e-3`** | **Polar wins `4.4× / 4.3×`** |
| **`(128, 53+11)`** | `3.0 dB` | `4.22e-2` vs. **`4.15e-3`** | `9.78e-3` vs. **`8.27e-4`** | **Polar wins `10.2× / 11.8×`** |
| **`(128, 53+11)`** | `3.5 dB` | `1.23e-2` vs. **`5.61e-4`** | `1.86e-3` vs. **`6.00e-5`** | **Polar wins `21.9× / 31.0×`** |
| **`(128, 88+11)`** | `4.0 dB` | `1.42e-2` vs. **`2.61e-3`** | `2.87e-3` vs. **`5.09e-4`** | **Polar wins `5.4× / 5.6×`** |
| **`(256, 82+11)`** | `3.0 dB` | `5.83e-2` vs. **`1.36e-4`** | `1.39e-2` vs. **`1.34e-5`** | **Polar wins `429× / 1,037×`** |

* **Why adding the same CRC flips the winner from RM to Polar at $n \ge 64$**:
  1. **Polar + CRC**: The 11-bit CRC acts as a high-distance outer filter across the $L$ list paths, rejecting $99.95\%$ ($1 - 2^{-11}$) of the weight-8 impostor codewords that plagued bare Polar. Once those low-weight impostors are removed, Polar reaps the full benefit of its better-polarized channels, improving at `(128, 53+11)` (`3.5 dB`) from `3.28e-3` (noCRC) to **`5.61e-4`** ($L=8$) and **`6.00e-5`** ($L=32$) despite paying a $0.82\text{ dB}$ rate tax ($53/128$ vs. $64/128$).
  2. **RM + CRC**: RM *already* had $d_{\min} = 16$, so it did not suffer from weight-8 list impostors; instead, RM's bottleneck under SCL is that its early weight-$(m-r)$ channels are poorly polarized and cause path splits that push the true path out of the list. Spending 11 of RM's 64 bits on an in-band CRC reduces the energy per coded bit by $10\log_{10}(64/53) \approx 0.82\text{ dB}$ without fixing those poorly polarized channels, causing RM's WER at `3.5 dB` to *degrade* from `8.50e-4` (noCRC) to `1.23e-2` (sameCRC at $L=8$).

#### C. Pure RM (No CRC) vs. Full 5G NR Uplink Polar (With CRC/PC) at Equal Payload (`RM → 5G`)

When pure RM (no CRC) is compared against 5G NR Uplink Polar (which must carry 6 or 11 CRC bits on top of the $k$-bit user payload), RM wins on raw word errors at $n \le 128$ because RM avoids the CRC rate tax—at the cost of $100\%$ silent (undetected) errors:

| Code $(n, k)$ | $E_b / N_0$ | $L = 8$ (`RM → 5G`) | $L = 32$ (`RM → 5G`) | Approx. Gain ($L = 8 \text{ / } L = 32$) |
| :--- | :---: | :---: | :---: | :--- |
| **`(32, 16)`** | `3.0 dB` | **`1.33e-2` → `4.44e-2`** | **`1.32e-2` → `1.75e-2`** | RM ahead `0.7 / 0.15 dB` |
| **`(64, 42)`** | `4.0 dB` | **`1.39e-3` → `3.08e-2`** | **`1.42e-3` → `9.06e-3`** | RM ahead `1.3 / 0.7 dB` |
| **`(128, 29)`** | `3.0 dB` | **`1.31e-3` → `5.45e-3`** | **`4.08e-4` → `1.22e-3`** | RM ahead `0.6 / 0.4 dB` |
| **`(128, 64)`** | `3.0 dB` | **`5.17e-3` → `8.34e-3`** | **`1.11e-3` → `1.65e-3`** | RM ahead `0.13 / 0.1 dB` |
| **`(128, 64)`** | `3.5 dB` | **`8.55e-4` → `1.30e-3`** | **`1.41e-4` → `1.46e-4`** | RM ahead slightly / tie |
| **`(128, 99)`** | `4.0 dB` | **`3.75e-3` → `2.34e-2`** | **`2.28e-3` → `6.96e-3`** | RM ahead `0.5 / 0.2 dB` |
| **`(256, 93)`** | `3.0 dB` | `1.34e-2` → **`1.45e-4`** | `1.91e-3` → **`1.21e-5`** | **5G ahead `~1 dB+`** |

#### D. Undetected Error Rate ($L = 8$)

| Code $(n, k)$, $E_b / N_0$ | Pure RM Undetected | 5G Polar Undetected | Same-CRC RM Undetected | Same-CRC Polar Undetected |
| :--- | :---: | :---: | :---: | :---: |
| **`(32, 16)`, `3.0 dB`** | `1.33e-2` | **`5.6e-3`** | `7.57e-3` (`10+6`) | `7.57e-3` (`10+6`) |
| **`(64, 42)`, `4.0 dB`** | `1.39e-3` | `1.4e-4` | `6.21e-5` (`31+11`) | **`3.96e-5`** (`31+11`) |
| **`(128, 64)`, `3.0 dB`** | `5.17e-3` | `3.9e-5` | `1.50e-4` (`53+11`) | **`1.76e-5`** (`53+11`) |
| **`(128, 64)`, `3.5 dB`** | `8.55e-4` | `7.5e-6` | `5.27e-5` (`53+11`) | **`4.09e-6`** (`53+11`) |
| **`(256, 93)`, `2.5 dB`** | `5.21e-2` | `1.2e-5` | `5.50e-4` (`82+11`) | **`4.93e-6`** (`82+11`) |

#### E. Average Decode Time per Word (Single Thread, AWGN incl. Clean Fast-Path)

| Code $(n, k)$ | $L = 8$: `RM / 5G` | $L = 32$: `RM / 5G` | Hard Voter (RM only) |
| :--- | :---: | :---: | :---: |
| **`(32, 16)`** | `3.5 / 6.3 µs` | `8.8 / 17.1 µs` | `0.44 µs` |
| **`(64, 42)`** | `9.8 / 12.5 µs` | `26.6 / 31.2 µs` | `0.99 µs` |
| **`(128, 64)`** | `19.1 / 22.8 µs` (`~3.4 Mb/s`) | `55.3 / 60.7 µs` (`~1.2 Mb/s`) | `2.30 µs` |
| **`(256, 93)`** | `36.9 / 39.2 µs` | `116.4 / 112.0 µs` | — |

---

### 2. Time & Memory Complexity vs. Other ECC Decoders

Let $n = 2^m$ be the block length, $k = \sum_{i=0}^r \binom{m}{i}$ be the message length, $r$ be the RM order, and $L$ be the list size.

| Decoder / Operation | Time Complexity | Space (Memory) Complexity | Concrete Cost at `(128, 64)` ($m=7, r=3, k=64$) |
| :--- | :--- | :--- | :--- |
| **`ecc-hypercube` Encode / Clean Check** | $\mathcal{O}(n \log n)$ | $\mathcal{O}(n)$ | $n \log_2 n = 896$ XORs (`encode`), $448$ XORs (`clean check`). |
| **`ecc-hypercube` Hard SWAR Voter** | $\mathcal{O}\!\left(\frac{k \cdot n}{64} + (m - r) n \log n\right)$ | $\mathcal{O}(n)$ (`~1.5 KB` tables) | $128$ `u64` masked popcounts + $3 \times 448$ butterfly XORs (`~2.3 µs`). |
| **`ecc-hypercube` Soft SC ($L=1$)** | $\mathcal{O}(n \log n)$ | $\mathcal{O}(n)$ (`~2.2 KB`) | $896$ `min_sum`/add ops + $448$ XORs (`~2.9 µs`). *(Asymptotically faster than the hard voter as $n \to \infty$!)* |
| **`ecc-hypercube` Soft SCL ($L$)** | $\mathcal{O}(L \cdot n \log n)$ | $\mathcal{O}(L \cdot n)$ (`17 L n` bytes: **`17.4 KB`** at $L=8$, **`69.6 KB`** at $L=32$) | $\le 896 L$ float ops + $64$ partial selects of size $2L$; fits entirely in L1/L2 cache with zero per-call heap allocations. |
| **5G Polar CA-SCL** *(Tal–Vardy)* | $\mathcal{O}(L \cdot n \log n)$ | $\mathcal{O}(L \cdot n)$ | Same $\mathcal{O}(L \cdot n \log n)$ butterfly + $\mathcal{O}(L \cdot k)$ CRC check at the end. |
| **Automorphism Ensemble (AED-$M$)** | $\mathcal{O}(M \cdot n \log n)$ *(parallel across $M$)* | $\mathcal{O}(M \cdot n)$ *(or $\mathcal{O}(n)$ sequentially)* | $M$ independent $\mathcal{O}(n \log n)$ SC decodes with zero path copying or sorting between paths. |
| **Recursive Projection-Aggregation (RPA)** | $\mathcal{O}(I_{\max} \cdot n^r \log n)$ | $\mathcal{O}(n^2)$ | Branches into $n - 1$ quotient sub-cubes at each of $r - 1$ levels ($\sim 127 \times 63 \times 31 \approx 248{,}000$ FHTs/iter at $r=3$). |
| **eBCH + OSD (Order-$p$)** | $\mathcal{O}\!\left(\frac{n^3}{64} + \sum_{i=0}^p \binom{k}{i} \frac{k(n-k)}{64}\right)$ | $\mathcal{O}(k \cdot n)$ | $\mathcal{O}(n^3)$ Gaussian elimination per word + $\binom{64}{p}$ candidate re-encodings ($41{,}664$ at $p=3$). |
| **GRAND / ORBGRAND** | $\mathcal{O}(Q_{\text{avg}} \cdot (n - k))$ | $\mathcal{O}(n)$ | Ultra-fast in VLSI hardware (`42–48 Gbps`) at high rate ($n - k \le 16$), but query count grows rapidly at $(128, 64)$ ($n - k = 64$). |

---

### 3. Positioning Against Modern ECC Alternatives

*Literature figures come from varying conditions and often dedicated ASIC/FPGA hardware; read this as architectural positioning rather than a direct CPU head-to-head.*

| Approach | Where It Stands | Compared with This Decoder |
| :--- | :--- | :--- |
| **LDPC** *(5G & reported 6G data channels)* | Dominant for long, high-throughput blocks. | Not a competitor: at long lengths only our hard decoder is practical (`~24 ms` at $m=15, p=7$). |
| **CRC-Aided Polar (CA-SCL)** *(5G control channel)* | Standardized and rate-flexible; hybrid decoders come within `~0.025 dB` of the finite-length limit at $N=128, R=1/2, \text{WER}=10^{-3}$. | • **No CRC (`noCRC`)**: Pure RM wins by `4×–30×` at $n = 64\text{--}128$.<br>• **With CRC (`ReedMuller::with_crc`)**: Ties 5G Polar at $n = 32, 64$ and `(128, 64+11)` (same code); **beats 5G Polar by `5–7×` at `(128, 64+6)`, $L=32$** (`4.5e-5` vs. `3.2e-4`, $A_8 = 0$); loses by `~2×` at `(128, 64+6/8)`, $L=8$ and at arbitrary non-RM rates (`sameCRC`) or $n \ge 256$. |
| **PAC Codes** *(Arıkan, 2019)* | Very strong at `(128, 64)`, especially with RM rate profiles. | Reuses RM's weight profile with convolutional precoding that eliminates RM's weight-16 floor—a natural upgrade path. |
| **eBCH + OSD** | Classic `(128, 64)` benchmark; $d_{\min} = 22$ vs. $16$ for $RM(3, 7)$. | Better minimum distance, though Ordered Statistics Decoding (OSD) is heavier; eBCH pulls ahead at very low error rates. |
| **GRAND / ORBGRAND** | Code-agnostic for short, high-rate codes; `~42–48 Gbps` in VLSI hardware at very low FER. | Far faster in dedicated hardware at high rates; not directly comparable to a single-thread CPU decoder at `~3.4 Mb/s`. |
| **Automorphism Ensemble Decoding (AED)** on $RM(3, 7)$ | Within `~0.04 dB` of ML at $10^{-3}$ WER. | Operates on the **exact same RM code**, at least as close to ML as $L=32$, and runs across independent parallel paths. |
| **Recursive Projection-Aggregation (RPA)** *(Ye & Abbe, 2020)* | Near-ML and beats Polar CA-SCL at short lengths. | Better gain at low order $r$, but requires $n - 1$ projections per recursion level ($\mathcal{O}(n^r)$), making it much heavier than SCL. |

---

### 4. Detailed Assessment: Practical, Coding-Theory, Engineering & Teaching Value

#### Point 1: `ReedMuller::with_crc` (No Weight-8 Codewords, Plus Error Detection, Without a Lookup Table)

* **Practical value for this library: High** *(turns a textbook code into one usable for packet links)*
  * **Before**: Plain RM had no error detection. Every decoding failure silently returned wrong bits (`Ok(wrong_msg)`), so a receiver could never request a retransmission. The naive way of adding a CRC—taking $c$ of RM's $k$ payload bits (Way 1)—made RM `2×–31×` worse than Polar with the same CRC.
  * **Now**: The $c$ most reliable weight-$(m-r-1)$ vertices are unfrozen, and the CRC goes on the last coefficient positions. This keeps the full $k$-bit payload and adds detection. Results at $n = 128$ with payload 64, `3.5 dB`:

| CRC bits | Failures caught, $L = 8$ | Failures caught, $L = 32$ | Word errors, $L = 32$ *(plain RM: `1.5e-4`)* |
| :---: | :---: | :---: | :--- |
| **6** | `86%` | `46%` | **`4.4e-5`** *(3.4× better than plain RM)* |
| **8** | `94%` | `59%` | `9.6e-5` |
| **11** | `99.5%` | `88%` | `1.1e-4` |

  * At $n = 64$ with `CRC-6` and $L = 8$, `80%` of failures are caught. `CRC-4` is not recommended: it leaves `12` weight-8 codewords and catches few errors.

* **Coding-theory value: Moderate** *(a clean, concrete demonstration of a known effect)*
  * At $n = 32$, $n = 64$, and `(128, 64+11)`, the construction produces the same information set as 5G Polar, so it performs identically; it is not a new code there.
  * At `(128, 64+6)` and $L = 32$, it is about `6×` better than Polar with the same 6-bit CRC (`4.4e-5` vs. `2.8e-4`):
    * Ranking positions by reliability alone (5G's sequence) drops RM vertices `15` and `23` for `97` and `56` at $K = 70$.
    * That raises the weight-8 codewords in the subspace from `432` to `816`, and **`19` of them survive the 6-bit CRC**.
    * The full `RM(3, 7)` basis plus 6 weight-3 vertices leaves **none** ($A_8 = 0$) after the CRC.
  * This is the known trade-off between channel reliability and minimum distance in finite-length list decoding, which motivates RM–Polar hybrids (Li–Shin–Tong 2014), Mondelli–Hassani–Urbanke (2014), and Trifonov's polar subcodes.
  * **Against real 5G NR**: For a 64-bit payload, 5G uses `CRC-11` ($K = 75$), which is the same code as `with_crc(3, 7, 11)` and performs equally. `CRC-6` at $L = 32$ has about `3.6×` fewer word errors than 5G (`4.4e-5` vs. `1.6e-4`), but catches about `46%` of failures against 5G's `~96%`.
  * **Position against the state of the art at `(128, 64)` near `1e-4` word errors** *(referenced against the Polyanskiy–Poor–Verdú normal approximation, which needs `2.92 dB` for `1e-4` at `(128, 64)`)*:

| Scheme | Gap to the Normal Approximation (`2.92 dB` at `1e-4` WER) |
| :--- | :--- |
| **This code, `CRC-6`, $L = 32$** | **`~0.45 dB`** |
| **This code, `CRC-6`, $L = 128\text{--}256$** | **`~0.15–0.25 dB`** *(few errors measured)* |
| **PAC codes / eBCH with OSD** *(from literature)* | `~0.1–0.2 dB` |
| **5G Polar, $L = 8$** | `~1.1 dB` |

  * **Vertex selection**: A 6-line $\beta$-expansion ([`beta_weight`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L729-L741), $\beta = 2^{1/4}$) reproduces 5G's order for the weight-3 vertices at $n = 128$ without a 1,024-entry table. It can be computed at any $n = 2^m$, though its ranking has primarily been validated at $n \le 256$.

---

#### Point 2: Three Decoders on One [`superset_xor`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L759-L772) Butterfly

* **Engineering value: High**
  * The syndrome and CRC fast path, Reed's majority-logic decoder (in its bit-packed 2013 form), and min-sum CA-SCL all share one superset-XOR transform.
  * The entire library is ~1,440 lines of dependency-free Rust ([`src/cube_code.rs`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs): 918 lines).
  * **Single-thread speeds at $n = 128$ with a 64-bit payload (`cargo bench --bench codec`)**:

| Decoder | Time per Word | Payload Throughput |
| :--- | :---: | :---: |
| **Clean-word fast path** | `< 1 µs` (`0.54 µs` at $n = 256$) | n/a |
| **Hard majority decoding** | `2.4 µs` *(plain RM)* | `≈ 27 Mb/s` |
| **Soft SCL, $L = 8$** | `13.4 µs` | `≈ 4.8 Mb/s` |
| **Soft SCL, $L = 32$** | `47.5 µs` | `≈ 1.35 Mb/s` |

  * Unifying RM and Polar decoding on the Arıkan kernel is established practice (e.g., AFF3CT); the contribution here is the hypercube formulation and a compact implementation.
  * **Hard-decision guarantee**: With a CRC, the guaranteed hard-decision correction radius drops to $2^{m-r-2} - 1$ errors (`3` for `RM(3, 7)`), which [`ReedMuller::error_tolerance()`](file:///Users/buitue/projects/ecc-hypercube/src/reed_muller.rs#L58-L64) reports.

* **Tiered (Adaptive-SCL) Decoding** *(Li et al., 2012)*
  Running $L = 8$ first and escalating to $L = 32$ only on `Err(Error::CrcMismatch)` only works when the CRC catches nearly all $L = 8$ failures, because any wrong frame that passes the CRC at $L = 8$ is never retried:
  * **`CRC-11`: Works.** $L = 8$ undetected errors are `6.7e-6`, so the tiered result stays close to $L = 32$'s `1.1e-4` at roughly $L = 8$ average speed.
  * **`CRC-6`: Does not work.** $L = 8$ undetected errors are already `1.16e-4`, so the tiered result is worse than running $L = 32$ directly (`4.4e-5`).
  * Escalating from hard `decode()` has not been measured (and at low SNR a short CRC would suffer high false-pass rates on single hard-decoded candidates, unlike the built-in `< 1 µs` clean-word fast path which checks all $n - k - c$ frozen parity vertices plus the CRC).

* **Teaching value: Very High**
  * Seeing that the 2013 unit-cube parity constraint, the Plotkin $(u, u \oplus v)$ split, and the Arıkan polar butterfly are the exact same involution makes list decoding much easier to follow. The algebraic equivalence is known (Arıkan 2009), but the hypercube view is an unusually direct way to see it.

---

#### Bottom Line & Caveats
* **Point 1**: `with_crc` adds error detection and, at `(128, 64+6)`, a better code than Polar with the same 6-bit CRC because the full RM basis removes every weight-8 codeword ($A_8 = 0$).
  * *For the lowest error rate*: `CRC-6` at $L = 32$ (`4.4e-5`, `~46%` of failures caught).
  * *For reliable detection*: `CRC-11`, which is 5G's own code and performs identically (`99.5%` caught at $L = 8$, `88%–96%` at $L = 32$).
  * *At $L \ge 128$*: Within `~0.15–0.25 dB` of the normal approximation limit, alongside PAC and eBCH with OSD; at $L = 8$, it is no better than 5G Polar.
* **Point 2**: One compact, dependency-free butterfly runs all three decoders; tiered decoding gives near-$L = 32$ error rates at near-$L = 8$ speed with `CRC-11`, but not with `CRC-6`.
* **Caveats**: Benchmarks cover AWGN only at fixed RM binomial dimensions ($k = \sum_{i=0}^r \binom{m}{i}$) and specific $E_b/N_0$ points; the $L = 128\text{--}256$ figures rest on few error events, and the PAC / eBCH / OSD figures are literature values.

---

## 11. Exploring CRC Integration for Future Work

### 1. How CRC Is Integrated Across Modern ECC Families

1. **Passive Outer Check (LDPC & Turbo Codes in 4G/5G Data Channels, Wi-Fi, Ethernet)**:
   * Sender appends a 16- or 24-bit CRC to the payload before encoding.
   * The belief-propagation decoder does **not** use the CRC to guide decoding (except as an early-stopping check between iterations); the CRC exists strictly to detect failed decodes and trigger HARQ retransmission.
2. **List-Pruning Outer Code: CA-SCL (5G NR Polar Codes — Tal & Vardy, 2015)**:
   * Sender appends a $c$-bit CRC ($c = 6, 11, 24$) to the $A$-bit payload and places all $K = A + c$ bits on the $K$ most reliable polar bit-channels.
   * At the receiver, the SCL decoder maintains $L$ candidate paths. At the end of the block, it sorts the $L$ paths by path metric and **selects the first path whose $c$-bit CRC passes** (or declares a detected failure if none pass).
   * Thus the CRC performs **two jobs simultaneously**:
     1. **Coding gain (error correction)**: Weeds out low-weight impostor codewords in the list, boosting effective $d_{\min}$ by `1–2 dB`.
     2. **Error detection**: Reduces the undetected error rate by a factor of $\sim 2^{-c}$ (`~130×` to `~4,300×` in the 5G benchmark above).
3. **Distributed / Interleaved CRC & Rolling Parity-Check (`PC`) Bits (5G NR Downlink PDCCH & Uplink PUCCH)**:
   * Instead of waiting until the final leaf of the tree to check the CRC, 5G downlink (`CRC24C`) interleaves CRC bits among the data bits so individual CRC bits can be verified **mid-tree**, pruning bad branches early before they evict the true path from a small list.
   * Similarly, for short uplink payloads ($12 \le A \le 19$, such as `(32, 16)` in [`benches/polar_5g.rs`](file:///Users/buitue/projects/ecc-hypercube/benches/polar_5g.rs#L114-L165)), 5G places 3 rolling 5-tap shift-register Parity-Check (`PC`) bits inside the tree to penalize wrong branches mid-flight.

---

### 2. Two Ways to Add a CRC to `ecc-hypercube` (and the Discrete-Rate Catch-22)

Unlike Polar codes—which can unfreeze any number of channels $K = A + c$ by picking the next $c$ entries in the reliability table—pure Reed–Muller codes only exist at discrete binomial dimensions ($k \in \{1, 8, 29, 64, 99, 120, 127\}$ at $n = 128$). That creates two distinct design options for adding a $c$-bit CRC to `ecc-hypercube`:

```
OPTION A: In-Band CRC (Pure RM, Payload = k - c)
  [ (k - c) User Bits | c CRC Bits ]  --->  k RM Data Vertices (weight >= m - r)
                                      --->  n - k Parity Vertices (weight < m - r, frozen to 0)

OPTION B: Out-of-Band / Hybrid RM-Polar CRC (Payload = k, Active = k + c)
  [ k User Bits ]  --->  k RM Data Vertices (weight >= m - r)
  [ c CRC Bits  ]  --->  c Strongest Weight-(m - r - 1) Vertices (unfrozen!)
                   --->  Remaining n - k - c Parity Vertices (frozen to 0)
```

| Design Option | How It Works in `SclDecoder` | Pros | Cons / Trade-offs |
| :--- | :--- | :--- | :--- |
| **Option A: In-Band CRC** *(Pure RM, $A = k - c$, tested in `sameCRC` above)* | Keep `CubeCode` / `ReedMuller` unchanged ($k$ data bits). Pack $k - c$ user bits + $c$ CRC bits into the $k$ data bits. In `SclDecoder::decode`, iterate over `0..self.active` in ascending metric order and return the first path whose $c$-bit CRC passes (or `Err(Error::CrcMismatch)` if none pass). | • Preserves pure RM structure and $d_{\min} = 2^{m-r}$.<br>• Drops undetected WER by `~100×` (`8.50e-4` $\to$ `5.27e-5` at `(128, 53+11)` `3.5 dB`).<br>• Zero changes to `SclDecoder::step`. | • **Tested in `sameCRC` (§10.1B)**: Paying the $10\log_{10}(k/(k-c))$ dB rate tax (`0.82 dB` for $53+11$) while keeping RM's unpolarized early weight-$(m-r)$ channels hurts RM's total WER (`8.50e-4` $\to$ `1.23e-2` at $L=8$), causing RM + CRC to lose to Polar + CRC by `2×–31×` at $n = 64\text{--}128$. |
| **Option B: Out-of-Band / Hybrid RM-Polar CRC** *(Payload $A = k$, Active $= k + c$)* | Keep all $k$ RM data vertices for the user payload, and **unfreeze the $c$ most reliable weight-$(m-r-1)$ parity vertices** (highest cube indices / 3GPP reliability) to carry the $c$-bit CRC. Because the CRC is checked on the coefficients/message bits, run `superset_xor` on each surviving path to verify the CRC. | • Preserves the full $k$-bit user payload (e.g., full 64 user bits at $n = 128$).<br>• Can eliminate RM's weight-16 impostor codewords while catching `>99%` of undetected errors. | • Unfreezing $c$ weight-$(m-r-1)$ vertices lowers the inner code's minimum distance from $2^{m-r}$ to $2^{m-r-1}$, requiring the CRC (and $L \ge 8..32$) to prune those lower-weight paths. |
| **Option C: Mid-Tree Parity-Check (`PC`) / PAC Pre-Transform** | Instead of leaving all $k$ data vertices independent, drive $c$ of the lowest-reliability data vertices (or a rate-1 convolutional pre-transform $v = u \ast g$ as in Arıkan's PAC codes) from a rolling shift register of prior data bits. | • Prunes bad branches **during** tree traversal rather than waiting until leaf $n-1$.<br>• PAC precoding on RM rate profiles reaches the finite-blocklength limit at `(128, 64)` (`~0.5 dB` gain over ML RM). | • Makes encoding non-systematic (requires running `superset_xor` after decoding to recover the message bits $u$). |

---

### 3. Implemented Option B / Way 2 (`ReedMuller::with_crc` / `CubeCode::with_crc`) Benchmark Results

We tested both Option A (**Way 1**: stealing $c$ of the $k$ RM data bits for the CRC, which shrinks the payload to $k - c$) and Option B (**Way 2**: keeping all $k$ RM data bits for the payload and unfreezing the $c$ most reliable weight-$(m-r-1)$ parity vertices):

* **Why Coefficient-Domain CRC on the Highest-Index Vertices Wins by `2×` ($A_8 = 0$)**:
  1. **Where the CRC Sits Changes the Code's Weight-8 Distance Spectrum ($A_8$)**:
     Unfreezing $c$ weight-$(m-r-1)$ vertices expands the unconstrained linear subspace from dimension $k$ ($d_{\min} = 16$ at $n = 128$) to dimension $k + c$ ($d_{\min} = 8$, introducing `432` weight-8 codewords for $c = 6$ and `816` weight-8 codewords for $c = 8$). Because the $c$-bit CRC is an $\mathbb{F}_2$-linear constraint, the valid CRC-passing codewords form a $k$-dimensional linear subcode inside that $(k+c)$-dimensional space—and **which** $c$ coordinates hold the CRC determines which hyperplane slices the space:
     * **CRC on the last (highest-index) unfrozen vertices in the coefficient ($u$) domain** (with the first $k$ unfrozen vertices—including the weight-3 vertices—carrying the payload): **eliminates every single weight-8 codeword (`A8 = 0` out of `432` at CRC-6, and `A8 = 0` out of `816` at CRC-8)**!
     * **CRC on the weight-3 vertices themselves**: leaves **`A8 = 5`** (in the $u$-domain) or **`A8 = 3`** (systematic in the codeword domain) weight-8 codewords unscreened at CRC-6, costing **`~2.2×` in total WER and `4–5×` in undetected WER** at $L = 32$ (`1.2e-4` / `1.4e-4` vs. **`4.5e-5` / `5.5e-5`**).

| Where the CRC Goes (`(128, 64+c)`, same info set, $3.5\text{ dB}$, $L = 32$) | Weight-8 Codewords ($A_8$, CRC-6 / CRC-8) | CRC-6 WER (`Undetected`) | CRC-8 WER (`Undetected`) |
| :--- | :---: | :---: | :---: |
| **Last (highest-index) $u$-positions; weight-3 vertices carry payload (`CubeCode::with_crc`)** | **`0 / 0`** *(out of 432 / 816)* | **`4.5e-5 – 5.5e-5` (`2.3e-5 – 3.0e-5`)** | **`8.3e-5 – 1.0e-4` (`2.5e-5 – 4.5e-5`)** |
| **On the weight-3 vertices in the $u$-domain** | `5 / 6` | `1.4e-4` (`1.1e-4`) | `1.7e-4` (`1.1e-4`) |
| **Systematic on the weight-3 vertices in the codeword domain** | `3 / 2` | `1.2e-4` (`9.4e-5`) | `1.1e-4` (`6.2e-5`) |

  2. **Preserving the 2013 Hard-Decision Majority Decoder (`CubeCode::decode`)**:
     In the superset transform `superset_xor`, a parity cube sum for a vertex `fixed` of weight $w$ sums `coefficients[u]` over $\text{base} \subseteq u \subseteq \text{fixed}$, which only depends on $u$ with $\operatorname{wt}(u) \le w$. Thus [`CubeCode::decode`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L254-L343) decodes the $c$ unfrozen weight-`p_dims` vertices **first** by majority vote over their $2^{p\_dims} - 1$ parallel cubes (threshold $2^{p\_dims - 1}$), strips their subset contributions from `code`, and then runs the original $2^{p\_dims + 1} - 1$ majority-vote loop across `weight >= p_dims + 1` before verifying the coefficient-domain CRC via [`extract_and_check_u`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L345-L361).
  3. **Table-Free 5G Channel Selection via $\beta$-Expansion (`beta_weight`, $\beta = 2^{1/4}$)**:
     [`beta_weight`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L684-L698) ($W(v) = \sum_{j : (v \gg j)\&1=1} (2^{1/4})^j$, He et al. 2017) ranks the weight-$(m-r-1)$ vertices without a lookup table, selecting **`[112, 104, 100, 88, 98, 84, 97, 56]`** at $n = 128$—matching the 3GPP 5G NR `Q_NMAX` reliability sequence for the top 10 weight-3 vertices at $n = 128$ and all 11 at $n = 256$.

#### Benchmark at Payload 64 (`(128, 64+c)` at $E_b / N_0 = 3.5\text{ dB}$ — `BENCH_MODE=rmcrc cargo bench --bench polar_5g` & `h5`)

| Code $(128, 64+c)$ | $L = 8$ Total WER (`Undetected`) | $L = 32$ Total WER (`Undetected`) | Takeaway |
| :--- | :---: | :---: | :--- |
| **`RM(3, 7)`, no CRC (`64+0`)** | `9.00e-4` (`9.00e-4`) | `1.53e-4` (`1.53e-4`) | Baseline ($100\%$ silent errors) |
| **Way 2: `RM + 4w3`, CRC-4 (`64+4`)** | `9.74e-4` (`4.65e-4`) | `2.30e-4` (`2.30e-4`) | Beats Polar `K=68` (`1.48e-3`), but 4-bit CRC leaves $A_8 = 12$ weight-8 words |
| **Polar `K=68`, CRC-4 (`64+4`)** | `1.60e-3` (`1.57e-3`) | `1.48e-3` (`1.48e-3`) | Stalls due to weight-8 words passing 4-bit CRC |
| **Way 2: `RM + 6w3`, CRC-6 (`64+6`)** | `8.77e-4` (`1.39e-4`) | **`4.53e-5` (`3.02e-5`)** | **Best $L=32$ code at payload 64 (`A8 = 0`; `3.4×` better than plain RM, `7×` better than Polar `K=70`)!** |
| **Polar `K=70`, CRC-6 (`64+6`)** | **`4.75e-4`** (`2.82e-4`) | `3.17e-4` (`2.96e-4`) | Dropping `{15, 23}` helps at $L=8$, but 2 extra weight-3 rows hurt at $L=32$ |
| **Way 2: `RM + 8w3`, CRC-8 (`64+8`)** | `8.92e-4` (`5.50e-5`) | **`8.33e-5 – 1.05e-4` (`2.45e-5 – 4.49e-5`)** | Beats or ties Polar `K=72` at $L=32$ (`1.24e-4`) with `2.4×` fewer silent errors |
| **Polar `K=72`, CRC-8 (`64+8`)** | **`3.73e-4`** (`4.63e-5`) | `1.00e-4 – 1.24e-4` (`7.28e-5 – 1.09e-4`) | Polar's sweet spot for payload 64 |
| **Way 2 `CRC-11` == Polar `K=75, CRC-11` (`64+11`)** | `1.34e-3` (`5.57e-6`) | `1.39e-4` (`~9e-6`) | Identical information set (`RM(3,7)` + 11 best weight-3 rows == top 75 of 5G sequence) |

---

## 12. Credits & Historical Lineage

The recursive soft-decision list decoder in `ecc-hypercube` stands on the shoulders of several foundational papers in coding theory:

1. **Reed–Muller Codes & Sub-Cube Majority-Logic Decoding (1954)**:
   * **D. E. Muller**, *"Application of Boolean algebra to switching circuit design and to error detection"*, IRE Trans. Electronic Computers, 1954.
   * **I. S. Reed**, *"A class of multiple-error-correcting codes and the decoding scheme"*, IRE Trans. Information Theory, 1954.
   * *Contribution*: Introduced Reed–Muller codes and hard-decision majority voting over disjoint affine subspaces (parallel sub-cubes).

2. **The Recursive $(u \mid u \oplus v)$ Cube-Splitting Construction (1960)**:
   * **M. Plotkin**, *"Binary codes with specified minimum distances"*, IRE Trans. Information Theory, 1960.
   * *Contribution*: Showed that an $RM(r, m)$ codeword of length $2^m$ splits into two half-length sub-cubes $(u \oplus v, u)$ where $u \in RM(r-1, m-1)$ and $v \in RM(r, m-1)$.

3. **Recursive Soft & List Decoding of Reed–Muller Codes (1999–2006)**:
   * **I. Dumer**, *"Recursive decoding of Reed–Muller codes"*, IEEE Trans. Information Theory, 2004 / 2006.
   * **I. Dumer and K. Shabunov**, *"Soft-decision decoding of Reed–Muller codes: Recursive lists"*, IEEE Trans. Information Theory, 2006 (first presented at IEEE ISIT 2001).
   * *Contribution*: Invented recursive soft-decision list decoding for Reed–Muller codes—recursively folding the hypercube along the Plotkin butterfly, decoding the lower half first, cancelling it into the upper half, and maintaining $L$ candidate paths at the leaves (predating polar codes by several years).

4. **Channel Polarization & Successive Cancellation List (SCL) Decoding (2009–2015)**:
   * **E. Arıkan**, *"Channel polarization: A method for constructing capacity-achieving codes for symmetric binary-input memoryless channels"*, IEEE Trans. Information Theory, 2009.
   * **I. Tal and A. Vardy**, *"List decoding of polar codes"*, IEEE Trans. Information Theory, 2015 (IEEE ISIT 2011).
   * *Contribution*: Formulated the recursive $f$ / $g$ butterfly tree with frozen (`0`) parity leaves and in-place path-slot management (copying state only when a surviving path splits into two children).

5. **LLR-Domain Min-Sum SCL & Piecewise-Linear Path Metrics (2014–2015)**:
   * **A. Balatsoukas-Stimming, M. B. Parizi, and A. Burg**, *"LLR-based successive cancellation list decoding of polar codes"*, IEEE Trans. Signal Processing, 2015.
   * *Contribution*: Formulated SCL entirely in the LLR domain using the min-sum Box-Plus update $f(L_1, L_2) = \operatorname{sign}(L_1)\operatorname{sign}(L_2)\min(|L_1|, |L_2|)$ and the exact LLR path metric penalty $\ln(1 + e^{-s \cdot L})$ together with its fast piecewise-linear approximation (`if l < 0 { metric -= l }`).

6. **The Systematic Unit-Cube / Superset Formulation (`ecc-hypercube`, Tue Bui, 2013 & 2026)**:
   * *Contribution*: Defines the code directly on the boolean hypercube where vertices of weight $\ge m - r$ hold systematic data bits and vertices of weight $< m - r$ enforce zero XOR sum over their unit cubes (superset cones). Because the superset Möbius transform ([`superset_xor`](file:///Users/buitue/projects/ecc-hypercube/src/cube_code.rs#L483-L494)) is an involution over $\text{GF}(2)$ that coincides with the Plotkin/Arıkan butterfly, systematic encoding, syndrome checking, hard multi-level cancellation, and in-place SCL partial-sum combining (`u_lower ^= u_upper`) all share the exact same butterfly—recovering the systematic codeword vertices directly in-place without a post-decode re-encoding transform.
