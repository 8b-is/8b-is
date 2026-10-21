# transformational familiarity, absorbed — five processes, four hypotheses, six-arm protocol

*Flyxion's "Transformational Familiarity: Food Neophobia, Autonomy, and the Construction of Taste" (Independent Researcher, October 2026, published in `standardgalactic/laboratory` as `transformational_familiarity.pdf` / `transformational_familiarity.tex` with companion audio overview `Fixing_Picky_Eating_With_Food_Vector_Math.mp3`) reframes childhood food refusal from a unitary defect of the child into a history-dependent search problem over food category space $\mathcal{X}$ and household culinary optionality $O_\rho(I, \tau, K)$.*

---

## 1. Disaggregating "Picky Eating" (§2)

Behaviors that look identical at the table arise from at least five distinct mechanisms (plus a sixth clinical category, ARFID, requiring specialist care), and applying the intervention for one mechanism to another makes refusal worse:

| Mechanism | Description | Appropriate Response | Counterproductive Intervention |
|---|---|---|---|
| **1. Food Neophobia** | Adaptive omnivore caution toward novel foods (twin heritability $\approx 0.78$, Cooke et al. 2007) | Repeated low-pressure contact, enthusiastic modeling, transformational chains | Forced consumption or pressure |
| **2. Sensory Aversion** | Negative response to specific texture, bitterness, temperature, or aroma (Cermak et al. 2010) | Textural/dimensional accommodation ($w_t$-aware chains holding high-weight dimensions constant) | Mere repetition of the aversive texture |
| **3. Preference for Familiar Set** | Willingness to taste paired with stable preference for an established repertoire | Gradual family-resemblance expansion without pathologizing baseline breadth | Treating narrow current breadth as stalled development |
| **4. Appetite Variation** | Multi-day energy regulation across meals rather than within single meals | Predictable meal structure and tolerance of meal-to-meal variation | Treating satiety-driven refusal as preference rejection |
| **5. Resistance to Coercion (Reactance)** | Psychological reactance (Brehm 1966) and self-determination defense (Deci & Ryan 2000) against mealtime pressure (Batsell et al. 2002; Galloway et al. 2005) | Satter (1986) division of responsibility: adult governs *what/when/where*, child governs *whether/how much* | Contingent rewards, restriction, or hunger as coercion |

---

## 2. Formal Apparatus: Preparations, Chains & Culinary Optionality (§4 & §7)

1. **Preparation Vector & Child-Specific Metric**:
   $$x = (f, t, a, v, c) \in \mathcal{X}, \qquad d_w(x, y) = \sum_{k \in \{f, t, a, v, c\}} w_k \, \delta_k(x_k, y_k)$$
   where $f$ is flavor, $t$ texture, $a$ aroma, $v$ visual presentation, $c$ contextual familiarity, and $w_k \ge 0$ are child-specific weights.
2. **History-Dependent Approach Propensity**:
   $$A_j(x \mid h, s) = \Pr(Y_j = 1 \mid x, h, s)$$
   Two identical preparations $z$ elicit different responses when reached via different encounter histories $h_{\text{chain}} \neq h_{\text{direct}}$ (**exact `CT-000` history-dependence**: state $z$ alone is not continuation-sufficient).
3. **The Four Hypotheses**:
   - **Hypothesis 1 (Transformational Familiarity)**: $A_{j^*}(z \mid h_{\text{chain}}, s) > A_{j^*}(z \mid h_{\text{direct}}, s)$ under matched offers, adult modeling, and zero pressure.
   - **Hypothesis 2 (Participation)**: Active participation in constructing the intermediate preparations increases exploration over passive receipt of identical preparations, mediated by **procedural** and **causal** familiarity (§6).
   - **Hypothesis 3 (Dimension Specificity)**: Tolerance $\epsilon$ varies by dimension $k$; for high-$w_t$ children, holding texture constant while varying other dimensions outperforms uniform micro-steps that feel like "near-misses."
   - **Hypothesis 4 (Higher-Order Transfer)**: $A_{j^*}(z' \mid h_{\text{chain}}, s) > A_{j^*}(z' \mid h_{\text{direct}}, s)$ for an unrelated novel preparation $z'$ — learning that culinary variation itself is safe to explore.
4. **Household Culinary Optionality & Inventory Dynamics (§7)**:
   $$O_\rho(I, \tau, K) = \bigl| \{ [m] \in \mathcal{M}/{\sim_\rho} : \exists m' \in [m] \text{ producible from } I \text{ within } \tau \text{ given } K, \, m' \in \mathcal{A} \} \bigr|$$
   $$I_{t+1} = I_t + P_t - X_t + \Phi_t(X_t) - E_t - W_t$$
   Cooking skill $K$ expands $O_\rho(I, \tau, K)$ without increasing expenditure, turning yesterday's preparation $\Phi_t(X_t)$ (bounded by safe storage life $\sigma_i > 0$) into the natural substrate of tomorrow's chain step.

---

## 3. Six-Arm Falsifiable Experimental Protocol (§9)

To separate the chain mechanism from mere exposure, ingredient familiarity, unordered variety, and adult attention, Section 9 specifies a pre-registered 6-arm design:
1. **Arm 1**: Direct presentation of $z$ + attention-matched non-food activity.
2. **Arm 2**: Ordered candidate chain $x_0 \to \dots \to z$ (passive) + attention-matched activity.
3. **Arm 3**: Randomized-order chain (same preparations as Arm 2, permuted order).
4. **Arm 4**: Ordered chain (participatory cooking) + matched non-food plant activity.
5. **Arm 5**: Ordered chain (participatory cooking + ingredient cultivation).
6. **Arm 6**: Ingredient-matched control (component ingredients of $z$ in unrelated preparations that do not form a chain).

---

## 4. Source & Audio Links

- **TeX Source**: [`laboratory/transformational_familiarity.tex`](file:///Users/lodripeter/workspace/peterlodri-sec/laboratory/transformational_familiarity.tex)
- **Compiled PDF**: [`laboratory/transformational_familiarity.pdf`](file:///Users/lodripeter/workspace/peterlodri-sec/laboratory/transformational_familiarity.pdf)
- **Audio Overview**: [`laboratory/Fixing_Picky_Eating_With_Food_Vector_Math.mp3`](file:///Users/lodripeter/workspace/peterlodri-sec/laboratory/Fixing_Picky_Eating_With_Food_Vector_Math.mp3) ([transcript](file:///Users/lodripeter/workspace/peterlodri-sec/laboratory/Fixing_Picky_Eating_With_Food_Vector_Math.txt))
