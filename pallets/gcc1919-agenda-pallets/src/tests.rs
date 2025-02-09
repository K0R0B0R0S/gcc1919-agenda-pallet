use frame_support::{assert_noop, assert_ok};
use crate::mock::{new_test_ext, CustomPallet, RuntimeOrigin, Test};


#[test]
fn test_criar_contato() {
    new_test_ext().execute_with(|| {
        // Teste criando um contato válido
        let nome = String::from("John Doe");
        let telefone = String::from("123456789");
        let email = String::from("john.doe@example.com");
        let idade = 30;
        let data_aniversario = String::from("01/01/1990");
        let categoria = crate::Categoria::Amigo;

        assert_ok!(CustomPallet::criar_contato(
            RuntimeOrigin::signed(1),
            nome.clone(),
            telefone.clone(),
            email.clone(),
            idade,
            data_aniversario.clone(),
            categoria.clone(),
        ));

        // Verifica se o contato foi criado corretamente
        let contato = CustomPallet::contatos(1, 0).unwrap();
        let data_formatada = CustomPallet::convert_to_timestamp(data_aniversario.into_bytes()).unwrap();
        assert_eq!(contato.nome, nome.into_bytes());
        assert_eq!(contato.telefone, telefone.into_bytes());
        assert_eq!(contato.idade, idade);
        assert_eq!(contato.data_aniversario, data_formatada);
        assert_eq!(contato.categoria, categoria);

        // Verifica se o próximo ID foi incrementado
        assert_eq!(CustomPallet::contador_contatos(1), 1);
    });
}

#[test]
fn test_criar_contato_data_invalida() {
    new_test_ext().execute_with(|| {
        // Teste criando um contato com data inválida
        let nome = String::from("John Doe");
        let telefone = String::from("123456789");
        let idade = 30;
        let email = String::from("john.doe@example.com");
        let data_aniversario = String::from("32/13/1990");
        let categoria = crate::Categoria::Amigo;

        assert_noop!(
            CustomPallet::criar_contato(
                RuntimeOrigin::signed(1),
                nome.clone(),
                telefone.clone(),
                email.clone(),
                idade,
                data_aniversario.clone(),
                categoria.clone(),
            ),
            crate::Error::<Test>::DataInvalida
        );
    });
}

#[test]
fn test_atualizar_contato() {
    new_test_ext().execute_with(|| {
        // Cria um contato válido
        let nome = String::from("John Doe");
        let telefone = String::from("123456789");
        let email = String::from("john.doe@example.com");
        let idade = 30;
        let data_aniversario = String::from("01/01/1990");
        let categoria = crate::Categoria::Amigo;

        assert_ok!(CustomPallet::criar_contato(
            RuntimeOrigin::signed(1),
            nome.clone(),
            telefone.clone(),
            email.clone(),
            idade,
            data_aniversario.clone(),
            categoria,
        ));

        // Atualiza o contato com novas informações
        let new_nome = String::from("Jane Doe");
        let new_telefone = String::from("987654321");
        let new_email = String::from("john.doe@example.com");
        let new_idade = 31;
        let new_data_aniversario = String::from("02/02/1990");
        let new_categoria = crate::Categoria::Familiar;

        assert_ok!(CustomPallet::atualizar_contato(
            RuntimeOrigin::signed(1),
            0,
            new_nome.clone(),
            new_telefone.clone(),
            new_email.clone(),
            new_idade,
            new_data_aniversario.clone(),
            new_categoria.clone(),
        ));

        // Verifica se o contato foi atualizado corretamente
        let updated_contato = CustomPallet::contatos(1,0).unwrap();
        let data_formatada = CustomPallet::convert_to_timestamp(new_data_aniversario.into_bytes()).unwrap();
        assert_eq!(updated_contato.nome, new_nome.into_bytes());
        assert_eq!(updated_contato.telefone, new_telefone.into_bytes());
        assert_eq!(updated_contato.idade, new_idade);
        assert_eq!(updated_contato.data_aniversario, data_formatada);
        assert_eq!(updated_contato.categoria, new_categoria);
    });
}

#[test]
fn test_deletar_contato() {
    new_test_ext().execute_with(|| {
        // Cria um contato válido
        let nome = String::from("John Doe");
        let telefone = String::from("123456789");
        let idade = 30;
        let email = String::from("john.doe@example.com");
        let data_aniversario = String::from("01/01/1990");
        let categoria = crate::Categoria::Amigo;

        assert_ok!(CustomPallet::criar_contato(
            RuntimeOrigin::signed(1),
            nome.clone(),
            telefone.clone(),
            email.clone(),
            idade,
            data_aniversario.clone(),
            categoria.clone(),
        ));

        // Deleta o contato
        assert_ok!(CustomPallet::deletar_contato(RuntimeOrigin::signed(1), 0));

        // Verifica se o contato foi deletado
        assert!(CustomPallet::contatos(1, 0).is_none());
    });
}

#[test]
fn test_criar_compromisso() {
    new_test_ext().execute_with(|| {
        let titulo = String::from("Reunião de Trabalho");
        let data = String::from("10/02/2025");
        let hora = String::from("14:00");
        let prioridade = crate::Prioridade::Alta;
        let duracao = 60;

        assert_ok!(CustomPallet::criar_compromisso(
            RuntimeOrigin::signed(1),
            titulo.clone(),
            data.clone(),
            hora.clone(),
            prioridade.clone(),
            duracao,
        ));

        let compromisso = CustomPallet::compromissos(1, 0).unwrap();
        let data_formatada = CustomPallet::convert_to_timestamp(data.into_bytes()).unwrap();
        assert_eq!(compromisso.titulo, titulo.into_bytes());
        assert_eq!(compromisso.data, data_formatada);
        assert_eq!(compromisso.hora, hora.into_bytes());
        assert_eq!(compromisso.prioridade, prioridade);
        assert_eq!(compromisso.duracao, duracao);

        assert_eq!(CustomPallet::contador_compromissos(1), 1);
    });
}

#[test]
fn test_criar_compromisso_hora_invalida() {
    new_test_ext().execute_with(|| {
        let titulo = String::from("Reunião");
        let data = String::from("10/02/2025");
        let hora = String::from("25:00");
        let prioridade = crate::Prioridade::Baixa;
        let duracao = 30;

        assert_noop!(
            CustomPallet::criar_compromisso(
                RuntimeOrigin::signed(1),
                titulo,
                data,
                hora,
                prioridade,
                duracao,
            ),
            crate::Error::<Test>::HoraInvalida
        );
    });
}

#[test]
fn test_criar_compromisso_duracao_invalida() {
    new_test_ext().execute_with(|| {
        let titulo = String::from("Reunião");
        let data = String::from("10/02/2025");
        let hora = String::from("14:00");
        let prioridade = crate::Prioridade::Media;
        let duracao = 0;

        assert_noop!(
            CustomPallet::criar_compromisso(
                RuntimeOrigin::signed(1),
                titulo,
                data,
                hora,
                prioridade,
                duracao,
            ),
            crate::Error::<Test>::DuracaoInvalida
        );
    });
}

#[test]
fn test_atualizar_compromisso() {
    new_test_ext().execute_with(|| {
        let titulo = String::from("Chamada com Cliente");
        let data = String::from("15/02/2025");
        let hora = String::from("09:30");
        let prioridade = crate::Prioridade::Media;
        let duracao = 45;

        assert_ok!(CustomPallet::criar_compromisso(
            RuntimeOrigin::signed(1),
            titulo.clone(),
            data.clone(),
            hora.clone(),
            prioridade.clone(),
            duracao,
        ));

        let new_titulo = String::from("Chamada com Equipe");
        let new_data = String::from("16/02/2025");
        let new_hora = String::from("10:00");
        let new_prioridade = crate::Prioridade::Alta;
        let new_duracao = 60;

        assert_ok!(CustomPallet::atualizar_compromisso(
            RuntimeOrigin::signed(1),
            0,
            new_titulo.clone(),
            new_data.clone(),
            new_hora.clone(),
            new_prioridade.clone(),
            new_duracao,
        ));

        let updated_compromisso = CustomPallet::compromissos(1, 0).unwrap();
        let data_formatada = CustomPallet::convert_to_timestamp(new_data.into_bytes()).unwrap();
        assert_eq!(updated_compromisso.titulo, new_titulo.into_bytes());
        assert_eq!(updated_compromisso.data, data_formatada);
        assert_eq!(updated_compromisso.hora, new_hora.into_bytes());
        assert_eq!(updated_compromisso.prioridade, new_prioridade);
        assert_eq!(updated_compromisso.duracao, new_duracao);
    });
}

#[test]
fn test_deletar_compromisso() {
    new_test_ext().execute_with(|| {
        let titulo = String::from("Consulta Médica");
        let data = String::from("20/02/2025");
        let hora = String::from("15:00");
        let prioridade = crate::Prioridade::Baixa;
        let duracao = 30;

        assert_ok!(CustomPallet::criar_compromisso(
            RuntimeOrigin::signed(1),
            titulo.clone(),
            data.clone(),
            hora.clone(),
            prioridade.clone(),
            duracao,
        ));

        assert_ok!(CustomPallet::deletar_compromisso(RuntimeOrigin::signed(1), 0));
        assert!(CustomPallet::compromissos(1, 0).is_none());
    });
}