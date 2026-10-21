# continuous microsurgery, absorbed — fourteen propositions, four subsystems, three exit horizons

*Flyxion's "Continuous Microsurgery: The Ethics of Never Putting the Knife Down" (Independent Researcher, October 2026, 40 pp.) shifts medical ethics from the episode to the trajectory. When small, frequent, individually low-risk interventions maintain the body indefinitely — through a distributed auxiliary organ (`hepastitium`), an alternative metabolic interface (`umbilicus`), continuous skeletal adjustment, and autonomous micro-scale tissue repair — the ethical unit of analysis is no longer the single intervention $a_t$, because an indefinite sequence of beneficial steps transforms the physiological, institutional, and psychological conditions under which later steps are chosen.*

---

## 1. The Longitudinal Decision Model & The Four Subsystems

At discrete steps $t = 0, 1, 2, \dots$, the state is a triple:
$$s_t = (x_t, k_t, \theta_t) \in X \times K \times \Theta$$
where $x_t \in X$ is physiological state (viability region $V \subset X$), $k_t \in K$ is institutional/technical state (implanted hardware, proprietary calibration records, supply chains), and $\theta_t \in \Theta$ parameterizes the patient's evaluative commitments $u(s; \theta_t)$. The unsupported exit trajectory from $(x_t, k_t)$ under $a = 0$ has survival horizon:
$$\tau_{\text{exit}}(x_t, k_t) = \inf \{ m \ge 1 : x_{t+m}^{(0)} \notin V \}$$

| Subsystem | Quantitative Constraint | Proved Bound / Formula | Verified Numerical Audit (`crates/admissibility`) |
|---|---|---|---|
| **Hepastitium** (Distributed Auxiliary Organ, Prop. 1) | Redundancy extends unmaintained viability only logarithmically; common-cause failure $p_c > 0$ creates a hard ceiling | $t_\varepsilon(N) = -\frac{1}{\mu} \ln\!\left(1 - [1 - (\varepsilon - p_c)_+/q]^{1/(N-N_{\min}+1)}\right)$, $p_{\text{fail}}(t) \ge p_c$ | For $N_{\min}=10, \mu=0.01\text{ d}^{-1}, \varepsilon=10^{-3}, p_c=0$: $t_\varepsilon(10) = 69.56\text{ d}$, $t_\varepsilon(100) = 270.67\text{ d}$, $t_\varepsilon(1000) = 497.87\text{ d}$ |
| **Umbilicus** (Adult Metabolic Interface, Prop. 2) | Light use can worsen mass balance when native clearance $C_G(I_U)$ atrophies faster than artificial clearance compensates | $g(\theta) = \alpha \theta E_U - M_0 - \beta \theta + [C_G(\theta I_U^{\max}) + \theta C_U]\Delta C_m$; trap when $g'(0) = \alpha E_U - \beta + I_U^{\max}(C_G'(0) + C_U/\ldots)\Delta C_m < 0$ | Example 2: $g(0) = -0.05$, $g'(0) = -0.06 < 0$, $g(0.1) = -0.0527 < g(0)$, break-even $\theta^* \approx 0.6306$, $g(1.0) = +0.07$ |
| **Continuous Skeletal Adjustment** (§3.3) | Rate limited by the slowest-adapting tissue; reversal is slower than forward modification | $r \le \min_{\tau \in \{\text{bone, cart, lig, V, N}\}} \bar{r}_\tau$, $\tau_{\text{rev}}(\Delta q) \ge \frac{r_{\text{fwd}}}{r_{\text{rev}}}\tau_{\text{fwd}}(\Delta q)$ | Remodeling asymmetry $r_{\text{rev}} < r_{\text{fwd}}$ creates structural path-dependence |
| **Autonomous Micro-Repair** (§3.4) | Non-zero false-positive off-target rate $h > 0$ implies a positive damage floor below which repair agents do net harm | $D_{\text{floor}}(n) = \frac{h K}{r - h} \quad (r > h)$ | For $D < D_{\text{floor}}$, $\dot{D} > \Lambda(t) - \rho_0 D$ (net iatrogenic harm) |

---

## 2. The Fourteen Propositions

| Prop. | Name | Formal Statement | `8b-is` / `CT-000` Mapping |
|---|---|---|---|
| **Prop. 1** | **Logarithmic Redundancy & Common-Cause Ceiling** | Unmaintained safe interval $t_\varepsilon(N) = \frac{1}{\mu}\ln N + O(1)$ as $N \to \infty$ when $p_c = 0$, and $p_{\text{fail}}(t) \ge p_c$ for all $N$ | Capacity is not capability; common-mode correlation $p_c$ mirrors `DMA` common-mode noise floor $q$ |
| **Prop. 2** | **Umbilicus Mass-Balance & Atrophy Trap** | If $g(0) < 0 < g(1)$ and $g'(0) < 0$, light assistive use $\theta \in (0, \theta_{\text{dip}})$ worsens net balance before high-use $\theta > \theta^*$ restores it | Non-monotone transition valley: partial substitution degrades native clearance before synthetic capacity takes over |
| **Prop. 3** | **Trajectory Non-Enforceability by Local Rules** | Any history-dependent admissibility predicate $Q(h_t)$ with $h_t \neq h'_t$, $x(h_t) = x(h'_t)$, and $Q(h_t) \neq Q(h'_t)$ cannot be enforced by any state-only rule $\phi(x_t)$ | Direct instance of **Central Theorem (`CT-000`)** & Nerode coarsest congruent partition (`CT-015`) |
| **Prop. 4** | **Hazard Accumulation & Exposure Coupling** | $P(T_{\text{comp}} \le H) = 1 - \exp(-\int_0^H h(u)\,du)$; under constant hazard $\lambda$ and competing mortality $\mu_0$, expected life gained is $\Delta L = \frac{1}{\mu_0} - \frac{1}{\lambda + \mu_1}$ | Cumulative complication probability approaching $1$ as $H \to \infty$ is coupled to life gained; comparison requires competing risks |
| **Prop. 5** | **Concave Option Price of Lock-In** | With future discontinuation desire $V \sim F$ ($S = 1-F$), the option price $\Pi(\kappa) = \int_0^\kappa S(c_0+s)\,ds$ satisfies $\Pi'(\kappa) = S(c_0+\kappa) \ge 0$ and $\Pi''(\kappa) = -f(c_0+\kappa) \le 0$ | Marginal option price per unit of exit cost is highest on the **first** increments of lock-in ($\kappa \downarrow 0$) |
| **Prop. 6** | **Greedy Over-Authorization Ratchet** | If institutional state $k_{t+1} = k_t + \delta_t$ ($\delta_t > 0$) raises exit cost $c(x,k)$ and step benefits $b_t \to 0$ with $\sum b_t < \infty$, local authorization $p_t \le \alpha$ permits $N(T) = \sum_{t=0}^{T-1}(b_t - p_t L) - [c(x_T,k_T)-c(x_0,k_0)] < 0$ | Example 4: $b_t = 3/(t+1), p_t = 0.02/(t+1), L=5, \Delta c = 2 \implies N(4) = +7.433$, $N(10) = -9.040$ |
| **Prop. 7** | **Damage-Balance Grace Period & Alternation** | Under $\dot{D} = \Lambda_0 - (\rho_{\text{nat}} + u)\psi(D)$ with $\psi(D) = D(1 - D/D_c)$, the post-withdrawal grace period is $T(a) = \int_{D^*}^{D_c} \frac{dD}{\Lambda_0 - \rho_{\text{nat}} D(1-D/D_c)}$ ($a = \Lambda_0 - \rho_{\text{nat}} D_c / 4$) | §7.3 & §12.3: $T(0.05) = 11.76\text{ d}$, $T(0) = 8.0\text{ d}$, $T(-0.05) = 6.20\text{ d}$; duty cycle $\theta = 0.50 \implies \rho_{\text{ss}} = 0.080, T = 9.83\text{ d}, \tau_{\text{bio}} = 43.77\text{ d}$ |
| **Prop. 8** | **Three Exit Horizons** | $t^*_{\text{ind}} = \inf\{t : \tau_{\text{exit}}(x_t) < \tau_{\text{svc}} + \tau_{\text{bio}}(t)\} \le t^*_{\text{port}} = \inf\{t : \tau_{\text{exit}}(x_t) < \tau_{\text{port}}(k_t)\}$ | Vendor lock-in horizon $t^*_{\text{port}}$ (loss of present supplier fatal) vs biological dependence horizon $t^*_{\text{ind}}$ |
| **Prop. 9** | **Endogenous Preferences & Veto Bound** | A binary veto $v_t \in \{0,1\}$ carries $I(M; V_{1:n}) \le H(V_{1:n}) \le n$ bits; step-wise consent under $\theta_t = \Phi_t(\theta_0, a_{0:t-1})$ does not entail consent under $\theta_0$ or $\theta_T$ | Example 5: $u(A;\theta_0)=0, u(B;\theta_0)=2, u(C;\theta_0)=-1$, but shifted $\theta_1$ chooses $C$ ($2 > 1$), yielding $\theta_0$-regret |
| **Prop. 10** | **Closed-Loop Trait Control & Controllability** | Under $\theta_{t+1} - \theta^* = (I - \Gamma K)(\theta_t - \theta^*) + \xi_t$, initial character decays at spectral radius $\rho(I - \Gamma K) < \rho(I - \Gamma_0)$, and reachability is confined to $\text{Im}[B, AB, \dots, A^{d-1}B]$ | Kalman/Wonham decomposition: observable proxies can regulate surface compliance while leaving deep dispositions untouched |
| **Prop. 11** | **Threshold Pressure Without a Coercer** | With continuation utility $v_i + s f$ ($v_i \sim F$, $s > 0$), interior equilibria solve $f^* = 1 - F(-s f^*)$, with $f^*(s) > f^*(0) = 1 - F(0)$ | Example 6: $v \sim \mathcal{N}(-0.2, 1), s = 1.5 \implies f^*(0) = 0.4207$, $f^*(1.5) = 0.8633$ ($51.3\%$ continue only due to peer shift) |
| **Prop. 12** | **Autonomy Enlargement Inclusion** | The regime enlarges autonomy for the exit future $F_{\text{exit}}$ ($\mathcal{A}_1(\beta) \supseteq \mathcal{A}_0(\beta)$) iff $c_{\text{exit}}(x_1, k_1) \le B_1 - (B_0 - c_0)$ | Longevity enlarges real freedom only when exit costs stay within what the person can bear |
| **Prop. 13** | **Independence of the Three Protections** | `(P1)` Continuation Rights, `(P2)` Exit & Migration Rights, and `(P3)` Dependence Minimality have $2^3$ realizable truth profiles ($3$ witness regimes violate one while satisfying two) | None of the three rights can be derived from the other two |
| **Prop. 14** | **Three Freedoms & Governability** | Exit ($E$), Migration ($M$), and Reconstitution ($C$) have $6$ realizable profiles under $C \implies E$; governability $G = E \wedge M \wedge A$ holds without $C$ in $(E, M, \neg C)$ | Autonomy under reconstruction requires **governability of dependencies** ($E \wedge M \wedge A$), not anatomical return to baseline |

---

## 3. Executable Witness & Interactive Surface

- **Rust `CT-000` / `CT-015` Witness (`695649b`)**: [`8b-is-engine/crates/admissibility/src/governance_trilogy_instances.rs`](https://github.com/8b-is/8b-is-engine/blob/main/crates/admissibility/src/governance_trilogy_instances.rs)
- **Compiled PDF (40 pp.)**: [`pocoo.vaked.dev/papers/flyxion-trilogy-2026/continuous_microsurgery.pdf`](https://pocoo.vaked.dev/papers/flyxion-trilogy-2026/continuous_microsurgery.pdf)
- **Interactive Codex & Simulator**: [`pocoo.vaked.dev/demos/trilogy/flyxion-governance-autonomy-trilogy.html`](https://pocoo.vaked.dev/demos/trilogy/flyxion-governance-autonomy-trilogy.html)
