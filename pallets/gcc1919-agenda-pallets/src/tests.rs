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

// #[test]
// fn test_criar_compromisso() {
//     new_test_ext().execute_with(|| {
//         // Teste criando um compromisso válido
//         let titulo = b"Reuniao".to_vec();
//         let data = b"01/01/2025".to_vec();
//         let hora = b"14:00".to_vec();
//         let prioridade = crate::Prioridade::Alta;
//         let duracao = 60;

//         assert_ok!(CustomPallet::criar_compromisso(
//             RuntimeOrigin::signed(1),
//             titulo.clone(),
//             data.clone(),
//             hora.clone(),
//             prioridade.clone(),
//             duracao,
//         ));

//         // Verifica se o compromisso foi criado corretamente
//         let compromisso = CustomPallet::compromissos(0).unwrap();
//         assert_eq!(compromisso.titulo, titulo);
//         assert_eq!(compromisso.data, data);
//         assert_eq!(compromisso.hora, hora);
//         assert_eq!(compromisso.prioridade, prioridade);
//         assert_eq!(compromisso.duracao, duracao);

//         // Verifica se o próximo ID foi incrementado
//         assert_eq!(CustomPallet::next_compromisso_id(), 1);
//     });
// }

// #[test]
// fn test_atualizar_compromisso() {
//     new_test_ext().execute_with(|| {
//         // Cria um compromisso válido
//         let titulo = b"Reuniao".to_vec();
//         let data = b"01/01/2025".to_vec();
//         let hora = b"14:00".to_vec();
//         let prioridade = crate::Prioridade::Alta;
//         let duracao = 60;

//         assert_ok!(CustomPallet::criar_compromisso(
//             RuntimeOrigin::signed(1),
//             titulo.clone(),
//             data.clone(),
//             hora.clone(),
//             prioridade.clone(),
//             duracao,
//         ));

//         // Atualiza o compromisso com novas informações
//         let new_titulo = b"Conferencia".to_vec();
//         let new_data = b"02/01/2025".to_vec();
//         let new_hora = b"10:00".to_vec();
//         let new_prioridade = crate::Prioridade::Media;
//         let new_duracao = 90;

//         assert_ok!(CustomPallet::atualizar_compromisso(
//             RuntimeOrigin::signed(1),
//             0,
//             new_titulo.clone(),
//             new_data.clone(),
//             new_hora.clone(),
//             new_prioridade.clone(),
//             new_duracao,
//         ));

//         // Verifica se o compromisso foi atualizado corretamente
//         let updated_compromisso = CustomPallet::compromissos(0).unwrap();
//         assert_eq!(updated_compromisso.titulo, new_titulo);
//         assert_eq!(updated_compromisso.data, new_data);
//         assert_eq!(updated_compromisso.hora, new_hora);
//         assert_eq!(updated_compromisso.prioridade, new_prioridade);
//         assert_eq!(updated_compromisso.duracao, new_duracao);
//     });
// }

// #[test]
// fn test_deletar_compromisso() {
//     new_test_ext().execute_with(|| {
//         // Cria um compromisso válido
//         let titulo = b"Reuniao".to_vec();
//         let data = b"01/01/2025".to_vec();
//         let hora = b"14:00".to_vec();
//         let prioridade = crate::Prioridade::Alta;
//         let duracao = 60;

//         assert_ok!(CustomPallet::criar_compromisso(
//             RuntimeOrigin::signed(1),
//             titulo,
//             data,
//             hora,
//             prioridade,
//             duracao,
//         ));

//         // Deleta o compromisso
//         assert_ok!(CustomPallet::deletar_compromisso(RuntimeOrigin::signed(1), 0));

//         // Verifica se o compromisso foi deletado
//         assert!(CustomPallet::compromissos(0).is_none());
//     });
// }