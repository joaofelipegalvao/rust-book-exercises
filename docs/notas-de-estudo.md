# Minhas Notas de Estudo - Rust 🦀

## Cap 1: Introdução

### O que aprendi:
- Rust foca em **segurança de memória** e **performance**
- **Cargo** é meu melhor amigo para gerenciar projetos
- **Comandos que uso:** `cargo new`, `cargo run`, `cargo check`
- **Insight:** Rust compila para código nativo, por isso é rápido!

---

## Cap 2: Jogo de Adivinhação

### Conceitos que entendi:
- `String::new()` cria string vazia que posso modificar
- `io::stdin().read_line(&mut guess)` lê o que o usuário digita
- `match` é como switch/case, mas muito mais poderoso
- `loop` + `break` para repetir até dar certo

### Dúvida resolvida: 
Por que `&mut`? É para passar **referência mutável** (não copia a variável)

### Ah-ha moment: 
Rust me força a tratar erros explicitamente com `Result<Ok, Err>`

---

## Cap 3.1: Variáveis e Mutabilidade

### Regra de ouro: Tudo é imutável por padrão!

```rust
let x = 5;        // não posso mudar
let mut y = 5;    // posso mudar
const MAX: u32 = 100; // constante (sempre imutável)
```

### Shadowing vs mut:
- **mut:** mudo o valor da mesma variável
- **Shadowing:** crio nova variável com mesmo nome

**Por que gostei:** Me força a pensar se realmente preciso mudar algo

---

## Cap 3.2: Tipos de Dados

### Escalares que uso mais:
- `i32` (inteiro padrão), `f64` (float padrão)
- `bool` (true/false), `char` (1 caractere Unicode)

### Compostos:
- **Tupla:** `(i32, f64, char)` - tipos diferentes
- **Array:** `[i32; 5]` - mesmo tipo, tamanho fixo

### Pegadinha: 
Arrays verificam bounds! Se acessar posição inexistente = panic

### Dica pessoal: 
Uso `let (x, y, z) = tupla;` para desestruturar

---

## Cap 3.3: Funções

### Descoberta importante: 
Rust diferencia **statements** de **expressions**

```rust
fn soma(x: i32, y: i32) -> i32 {
    x + y  // sem ; = expression (retorna valor)
}

fn main() {
    let resultado = soma(5, 3); // statement
}
```

### Lição aprendida: 
Esqueci o `;` e virou return. Esqueci o `->` e deu erro.

### Blocos são expressions:
```rust
let y = {
    let x = 3;
    x + 1  // retorna 4
};
```

---

## Cap 3.4: Comentários

### Simples:
```rust
// comentário de linha
/* */ comentário de bloco
/// documentação (aprendi que gera docs automáticas!)
```

---

## Cap 3.5: Controle de Fluxo

### If deve ser bool: 
Rust não converte automaticamente (diferente de JS/Python)

```rust
// ❌ Errado
let numero = 3;
if numero {  // ERRO! Rust espera bool
    println!("número era 3");
}

// ✅ Certo  
let numero = 3;
if numero != 0 {
    println!("número era algo diferente de zero");
}
```

### If básico:
```rust
fn main() {
    let numero = 3;
    
    if numero < 5 {
        println!("condição era verdadeira");
    } else {
        println!("condição era falsa");
    }
}
```

### Múltiplas condições com else if:
```rust
fn main() {
    let numero = 6;
    
    if numero % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }
}
// Saída: "número é divisível por 3"
```

### If como expressão: 
Posso usar em `let`!

```rust
fn main() {
    let condicao = true;
    let numero = if condicao { 5 } else { 6 };
    
    println!("O valor do número é: {}", numero);
    // Saída: "O valor do número é: 5"
}
```

**⚠️ IMPORTANTE:** Ambos os braços do `if` devem retornar o mesmo tipo!

```rust
// ❌ ERRO - tipos incompatíveis
let numero = if condicao {
    5
} else {
    "seis"  // ERRO: esperado inteiro, encontrado &str
};
```

---

## Loops que entendi:

### 1. `loop`: infinito até break
```rust
fn main() {
    loop {
        println!("novamente!");
        // Ctrl+C para parar, ou usar break
    }
}
```

### 2. `while`: com condição
```rust
fn main() {
    let mut numero = 3;
    
    while numero != 0 {
        println!("{}!", numero);
        numero = numero - 1;
    }
    
    println!("LIFTOFF!!!");
}
// Saída: 3! 2! 1! LIFTOFF!!!
```

### 3. `for`: mais seguro para arrays
```rust
// ❌ Forma perigosa com while
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;
    
    while indice < 5 {  // E se mudar o tamanho do array? 💥
        println!("O valor é: {}", a[indice]);
        indice = indice + 1;
    }
}

// ✅ Forma segura com for
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    for elemento in a.iter() {
        println!("O valor é: {}", elemento);
    }
}
```

### Por que for é melhor: 
Não risco acessar índice inválido

### Range trucão: 
`(1..4).rev()` faz 3, 2, 1

```rust
fn main() {
    for numero in (1..4).rev() {
        println!("{}!", numero);
    }
    println!("LIFTOFF!!!");
}
// Saída: 3! 2! 1! LIFTOFF!!!
```

---

## 🤔 Minhas Dúvidas Atuais
- Como funciona **ownership**? (próximo capítulo!)
- Por que às vezes uso `String` e às vezes `&str`?
- O que são **lifetimes**?

---

## 💡 O que Mais Me Impressionou
1. **Rust me obriga a ser explícito** - no início irritante, depois percebi que evita bugs
2. **Compiler é meu professor** - erros são super explicativos
3. **Imutabilidade por padrão** mudou como penso sobre código
4. **Zero-cost abstractions** - alta level sem perder performance

---

## 🎯 Exercícios que Fiz

### 1. Hello World
```rust
fn main() {
    println!("Hello, world!");
}
```

### 2. Jogo de Adivinhação (conceitos principais)
```rust
use std::io;

fn main() {
    println!("Adivinhe o número!");
    
    loop {
        println!("Digite seu palpite:");
        
        let mut palpite = String::new();
        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler linha");
            
        // ... lógica do jogo
        break; // sair quando acertar
    }
}
```

### 3. Conversor Fahrenheit/Celsius
```rust
fn fahrenheit_para_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_para_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let temp_f = 100.0;
    let temp_c = fahrenheit_para_celsius(temp_f);
    println!("{}°F = {}°C", temp_f, temp_c);
}
```

### 4. Fibonacci (iterativo)
```rust
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

fn main() {
    for i in 0..10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}
```

### 5. Os Doze Dias de Natal
```rust
fn main() {
    let dias = [
        "primeiro", "segundo", "terceiro", "quarto", "quinto", "sexto",
        "sétimo", "oitavo", "nono", "décimo", "décimo primeiro", "décimo segundo"
    ];
    
    let presentes = [
        "uma perdiz numa pereira",
        "duas rolas",
        "três galinhas francesas",
        // ... continuar
    ];
    
    for dia in 0..12 {
        println!("No {} dia de Natal", dias[dia]);
        println!("meu amor me deu:");
        
        for presente in (0..=dia).rev() {
            println!("{}", presentes[presente]);
        }
        println!();
    }
}
```

---

## 📚 Resumo dos Pontos-Chave

### Controle de Fluxo:
- **if** sempre precisa de condição `bool`
- **if** pode ser usado como expressão em `let`
- **else if** para múltiplas condições
- Tipos dos braços `if/else` devem ser compatíveis

### Loops:
- **loop**: infinito até `break`
- **while**: executa enquanto condição for verdadeira  
- **for**: mais seguro para iterar coleções
- **Range**: `(1..4)` = 1,2,3 | `(1..4).rev()` = 3,2,1

### Boas Práticas:
- Prefira `for` over `while` para arrays
- Use `if` como expressão quando apropriado
- Aproveite a verificação de tipos do compilador

---

## 📅 Próximo Passo
**Cap 4: Ownership** - o conceito que define Rust!

Todo mundo fala que é o mais difícil, mas também o mais importante. Vou dedicar tempo extra para entender bem.

**Atualizado:** [28/06/2025]  
**Próxima revisão:** depois do Cap 4
