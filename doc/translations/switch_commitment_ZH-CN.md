# 秘诺切换简介

*阅读其它语言版本： [English](../switch_commitment.md)*

## 概述


密码学中，_秘诺_ （或_秘诺方案_）这个概念可以想像成一个加锁的盒子。可以在盒子里放东西（例如一张写着密码的纸），把盒子锁起来，再交给另一个人（或公示）。

另一个方不知道盒子里的密码，但是如果你决定之后按约定公开密码，并要证明这确实是最初你放在盒子里的密码（并未篡改），你仅把盒子钥匙交给另一个人即可证明。

另一个人可以打开盒子，对比盒子里的密码和你刚刚公开的密码，就可以确定盒子上锁后密码没有变更。你提前"**承诺**"密码，意味着在秘诺生成和公开之间，你不可以对其做任何修改。


## 范例

### 秘诺哈希计算

使用密码学哈希函数即可实现简单的秘诺方案 (commitment scheme)。例如，Alice 和 Bob 想要玩“猜数游戏”。Alice 想出自己的密数 `29`，由 Bob 来猜。在游戏开始前，Alice 做如下计算：

    hash( 29 + r )

并把结果告诉给 Bob。`r` 是随机选择的_“致盲因子 (Blinding Factor)”_。之所以需要致盲因子是因为如果没有，Bob 就可以尝试所有可能数字的哈希值，来对比原哈希值。

游戏结束时，Alice 仅需公开自己的密数 `29` 和盲因子 `r`。Bob 自己哈希计算结果，即可验证 Alice 在游戏过程中是否更改过密数。


### Pedersen Commitment

另外，高阶秘诺方案还有其他特性。例如 Mimblewimble 和机密交易 (Confidential Transactions，CT) 大量使用_同态 (homomorphic)_ 秘诺的 _[Pedersen Commitments](https://link.springer.com/content/pdf/10.1007/3-540-46766-1_9.pdf)_。这里的“同态”是指，引用上述“盒子”的比喻，可以使用两个加锁盒子（_盒子  1_ 和_盒子 2_），而且可以将其_“合并”_一起。这样你就得到一个大盒子（仍旧加锁），如果之后打开这个含有密数的大盒子（如上述范例），最终的结果就是_盒子  1_ 和_盒子 2_ 的密数总和。

这个“盒子”的比喻在现实世界中看似不合理，但对椭圆曲线原理无可挑剔。

如需了解关于 Pedersen Commitment 的详情，请参阅[《Mimblewimble 和 Mugle 简介》](intro_ZH-CN.md)


## 秘诺方案特性：

一般而言，秘诺有两大特性，强弱变化视秘诺方案类型而变。

- **隐匿性（或机密性）：**秘诺方案保护秘诺的强度。或依上述范例：攻击者没有钥匙怎么能打开盒子（知道密数）？

- **绑定性：**攻击者是否可以找到匹配同一秘诺的不同密数，攻击者之后通过秘诺打开的就是不同秘诺。这样秘诺的_致盲性_就被破解。

### 两种特性的安全性：

对于这两种特性，可以确认不同安全级别。

两种特性最重要的混合使用方式为

- **完美绑定**和**计算隐匿**秘诺方案，和
- **计算绑定**和**完美隐匿**

“_计算_”绑定或隐匿的基础数学难度很高，以现有计算能力在合理时间内难以破解（即以世界上的现有计算资源今天不可破解），因此意味着这种特性（绑定性／隐匿性）安全。

“_完美_”绑定或隐匿意味着即使有无限计算资源，也不可能破解这一特性（绑定性／隐匿性）



### 互不相容：

需要意识到重要的一点，秘诺方案**不可能**同时兼备_完美绑定_**和**_完美隐匿_。一个思想实验就可以明白：假设攻击者有无限算力，他可以对任意数值生成秘诺（和致盲因子），直到碰撞找到匹配原始秘诺的输出数值。进一步假设秘诺方案为_完美绑定_（意味着不可能两个不同的数值生成相同秘诺），匹配秘诺的数值独一无二，那这样就会破坏隐匿性。

反之亦然。如果一个秘诺方案为_完美隐匿_，那必然会有多个输入值生成相同秘诺（否则有无限算力的攻击者只需如上所示计算任意数值）。结果就是，这一秘诺方案不可能是_完美绑定_。

#### 有所取舍

此处的要点就是：**总要有所取舍**，无法兼备两种特性（_隐匿性_和_绑定性_）的_完美_安全性。如果一种是_完美_安全，那另一种至多就是_计算_安全（反之亦然）。


### 加密货币中的应用考虑

在设计加密货币时，这些特性扮演了哪些作用？

**隐匿性**：
对于 Mugle 这类注重隐私的加密货币，秘诺方案用以保护交易数据。发送者执行发送一定数量的币，但对于他人具体金额保密（秘诺方案中的_隐匿性_特性）。

**绑定性**：
同时，交易发起者之后也无法更改秘诺为不同的交易金额。如果更改秘诺成功，攻击者就可以花费比之前在一个 UTXO 生成的币多的金额，这样就凭空造出“假币”。更糟的是，由于金额隐藏，这种情况有可能无法被发现。

所以保持两种特性安全可靠是根本所在。

只要一个加密货币始终维持这两种特性，就得选择使用哪种秘诺方案。


#### 难以选择？

哪种特性需要_完美_安全，哪种足以_计算性_安全？换句话说：如果秘诺方案被意外攻破，应该提高哪种特性安全级别？是经济稳定性（无通胀可能）还是保证隐私性（隐私不被侵犯）？

这似乎是个难题。

深入了解，我们意识到秘诺方案被攻破时需要_完美_绑定。那时候即使只有_计算_绑定也会安全。

同时隐私加密货币需要**永远**保证_隐匿性_。_绑定_特性只有在创建交易且不会影响过去交易的情况下才重要。不同的是，需要始终保持_隐匿性_。否则遇到意外秘诺方案被攻破的情况下，攻击者会链上回滚，解绑历史交易，进而破坏隐私性。

## Pedersen Commitments 特性

Pedersen Commitments 是**计算绑定**和**完美隐匿**。预设秘诺值为 `v`: `v*H + r*G`，会存在一组不同的值 `r1` 和 `v1` 其和与预设秘诺值相同。即使有无限算力可以尝试所有可能的值，攻击者也无法分辨哪一个是原始秘诺（因而是_完美隐匿_）。


## 秘诺切换简介

如果 Pedersen Commitment 的绑定性被意外破解那会发生什么？

一般而言，加密货币的秘诺方案被破解，就会更改加密方案。但问题是更改加密方案就需要使用新的方案创建新的交易输出，来保证资金安全。这就需要每位持币者将币转到新的交易输出。如果没有转币到新交易输出，那就不会受到新秘诺方案的保护。而且一定要在就方案被大规模攻破**之前**转币，否则现有 UTXO 不会记录正确的交易值。

遇到这种情况，[_Switch Commitments_](https://eprint.iacr.org/2017/237.pdf) 给出了简洁的解决方案。这种秘诺仅靠更改公开／验证程序来变更秘诺特性，而无须变更秘诺创建方式。（“_切换_”到新验证方案，可与在“_切换_”倩生成的秘诺兼容。）


### 工作方案

首先来介绍新的秘诺方案：**ElGamal commitment** 方案。其为_完美绑定_（由于不能二者兼具所以是_计算隐匿_）。其特征与 Pedersen Commitments 极其类似，只是添加了新元素，致盲因子 `r` 与 `J` 相乘计算得出：

    v*H + r*G ,  r*J

如果我们储存额外的域 `r*J`，并且暂时忽略，我们可以将其视作 Pedersen Commitments 对待，未来随时可以激活完整 ElGamal commitment。主网上线前，[Mugle 早期版本](https://github.com/mugleproject/mugle/blob/5a47a1710112153fb38e4406251c9874c366f1c0/core/src/core/transaction.rs#L812)就是这样部署。详情为：哈希值 `r*J`
(_switch\_commit\_hash_) 添加到交易输出，但造成每个输出大小增加 32 字节。

幸运的是，之后 Mimblewimble 邮件列表成员 Tim Ruffing 提出一个[绝妙的解决方案](https://lists.launchpad.net/mimblewimble/msg00479.html)（最初是 Pieter Wuille 所建议）。这一方案保持了相同优势，但不会对交易输出造成额外体积负担。

方案内容如下：

普通的 Pedersen Commitment 是这样：

    v*H + r*G

（`v` 是输入／输出值，`r` 是随机致盲因子，`H` 和 `G` 是椭圆曲线上的两个生成点）。

如果加以更改，让 `r` 变为非随机数，但使用另一个随机数 `r`，来创建 Pedersen Commitment：

    v*H + r*G

例如：

    r = r' + hash( v*H + r'*G  ,  r'*J )

（使用椭圆曲线上的另外第三生成点 `J`），然后 `r` 因为仍旧随机分布，所以作为致盲因子仍完美有效，但我们现在看到的括号内哈希函数 (`v*H + r'*G  ,  r'*J`) 是 **ElGamal commitment**。

这一绝妙的方案就从输出中移除了秘诺切换哈希（详情请参阅 [pull requests](https://github.com/mugleproject/mugle/issues/998)），这样就可以轻松纳入 Pedersen Commitment。

这就是 Mugle 目前的秘诺部署方案。Pedersen Commitment 用作机密交易 (Confidential Transaction)，但没有单纯随机选择致盲因子 `r`，而是在一个随机数 `r` 添加 ElGamal commitment 函数来计算（详情请参阅 [main_impl.h#L267](https://github.com/mugleproject/secp256k1-zkp/blob/73617d0fcc4f51896cce4f9a1a6977a6958297f8/src/modules/commitment/main_impl.h#L267)）。

总之，秘诺切换是在论文[《秘诺切换：安全切换机密交易》](https://eprint.iacr.org/2017/237.pdf)中首次提出。“切换”一词来源于未来可以随意“扳动开关”这一想法。仅仅变更验证程序就可以更改秘诺绑定性和隐匿性特性强弱，未来的更改甚至可与创建的历史秘诺兼容。



## 结语

Mugle 和其他加密货币一样，都使用 Pedersen Commitments。唯一的区别就是随机致盲因子 `r` 是利用 ElGamal
commitment 方案生成。

这种方案看上去没有差别，但有重要的安全措施：

Pedersen Commitments 已经是_完美绑定_。所以无论发生什么，无需用户任何操作就可保证无隐私泄露风险。若遇到意外，秘诺方案的绑定性被破解，需要所有新交易通过验证所有 ElGamal 秘诺，证明他们的秘诺没有破解绑定性，就可以启用秘诺切换（通过软分叉）。

但这种情况下用户仍有选择：

- 用户可决定继续创建新交易。即使因为 ElGamal 秘诺方案仅为计算隐匿，有可能破坏隐私性（只对**上一个** UTXO），但用户至少仍旧可以存取自己的币。

- 或者用户可决定不管钱，不做任何交易（但是保留隐私性，因为他们的交易仅验证有完美隐匿性的 Pedersen 秘诺）

有些情况下，隐私泄露对某个人的生命安全威胁要高过一定加密货币的损失。但决策权应该留给个人用户，秘诺切换就实现了这种选择。

需要明确的是，这一安全措施只有在遇到意外灾难的情况下才会启用。若计算有所进步，离散对数难题受到质疑，那包括加密货币在内的众多其他加密系统都需要紧急更新原语，抵抗未来潜在威胁。秘诺切换只是在 Pedersen Commitments 被意外破解情况下，提供额外的安全保护方案。