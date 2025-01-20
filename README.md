## Documentação do Código FOLHARHGTKRELMSQLITE

Este documento fornece uma visão geral do código presente nos arquivos `conexao.rs`, `main.rs` e `Cargo.toml`. O projeto utiliza a biblioteca `rusqlite` para manipulação de um banco de dados SQLite e a biblioteca `gtk` para a interface gráfica.

### Estrutura do Projeto

- **conexao.rs**: Este arquivo contém a implementação da estrutura de conexão com o banco de dados e as operações CRUD (Criar, Ler, Atualizar, Deletar) para a tabela `CONTRATADOS`.
- **main.rs**: Este arquivo é responsável pela configuração da interface gráfica utilizando GTK e pela lógica de interação com o usuário.
- **Cargo.toml**: Este arquivo contém as dependências do projeto e as configurações do Cargo, o gerenciador de pacotes do Rust.

### Arquivo: conexao.rs

**Descrição das Funções:**
- `criar_tabela()`: Cria a tabela `CONTRATADOS` no banco de dados se ela não existir.
- `incluir()`: Insere um novo registro na tabela.
- `alterar()`: Atualiza um registro existente.
- `excluir()`: Remove um registro da tabela.
- `consultar_por_id()`: Recupera um registro específico pelo ID.

### Arquivo: main.rs

**Descrição das Funções:**
- O modelo inicializa a tabela no banco de dados.
- A função `update` processa diferentes eventos da interface, como cálculos e manipulação de registros.

### Arquivo: Cargo.toml

**Descrição das Dependências:**
- **gtk**: Biblioteca para a criação de interfaces gráficas.
- **relm**: Facilita a programação reativa em Rust com GTK.
- **rusqlite**: Fornece acesso ao SQLite.

### Instruções para Execução

1. **Instalação das Dependências**:
   Execute o comando abaixo para instalar as dependências necessárias:
   ```bash
   cargo build
   ```

2. **Execução do Programa**:
   Para rodar o programa, utilize:
   ```bash
   cargo run
   ```

3. **Interação com a Interface**:
   A interface permite calcular salários, adicionar registros e visualizar informações armazenadas no banco de dados.

### Conclusão

Esta documentação fornece uma visão geral do código e suas funcionalidades principais. Para mais detalhes sobre cada função ou método, consulte os comentários no código-fonte.
