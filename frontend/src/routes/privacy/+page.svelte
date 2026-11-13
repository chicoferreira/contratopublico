<script lang="ts">
  import ProsePage from "$lib/components/ProsePage.svelte";
</script>

<ProsePage
  title="Dados e Privacidade"
  description="Política de privacidade e proteção de dados do Contrato Público.">
  <h1>Política de Privacidade e Proteção de Dados</h1>

  <p><em>Última atualização: Novembro de 2026</em></p>

  <h2>Sobre os Dados dos Contratos</h2>

  <h3>Origem dos Dados</h3>

  <p>
    Todos os dados de contratos públicos disponibilizados no <strong>Contrato Público</strong> são
    recolhidos exclusivamente do <a href="https://www.base.gov.pt/base4">Portal BASE</a> e da
    respetiva <a href="https://www.base.gov.pt/APIBase2V2">API</a>.
  </p>

  <p>
    O <strong>Contrato Público</strong> atua como um agregador e motor de pesquisa, facilitando o acesso
    e pesquisa destes dados públicos através de uma interface mais rápida, intuitiva e funcional. Não
    somos afiliados ao IMPIC nem ao Governo de Portugal.
  </p>

  <h3>Apresentação dos Dados</h3>

  <p>
    Os dados são apresentados conforme disponibilizados no Portal BASE. O <strong
      >Contrato Público</strong> não altera o significado nem os valores dos contratos; pode apenas normalizar
    a forma de apresentação (por exemplo, formatos de datas/valores ou separação de códigos CPV) para
    melhorar a leitura e pesquisa. Não valida a exatidão ou atualidade das informações, não adiciona conteúdo
    que não exista no Portal BASE e limita-se a agregar e indexar para proporcionar uma experiência de
    pesquisa mais eficiente.
  </p>

  <h3>Responsabilidade e Limitações</h3>

  <p>
    O <strong>Contrato Público</strong> procura manter os dados alinhados com o Portal BASE, mas não garante
    atualizações em tempo real, nem assume responsabilidade por erros, imprecisões ou atrasos presentes
    na fonte. Para confirmação de informações críticas, consulte sempre diretamente o Portal BASE.
  </p>

  <h3>Correções e Atualizações</h3>

  <p>
    Pedidos de correção, atualização ou remoção de dados relativos a contratos públicos devem ser
    submetidos diretamente ao <strong>IMPIC</strong>. Os contratos removidos do Portal BASE e da
    respetiva API são também removidos automaticamente do <strong>Contrato Público</strong>. Se já
    tiver solicitado a alteração ou remoção e esta estiver refletida no Portal BASE há mais de 7
    dias mas ainda não no <strong>Contrato Público</strong>, contacte-nos via
    <a href="mailto:contacto@contratopublico.pt">contacto@contratopublico.pt</a> para procedermos à atualização
    manual.
  </p>

  <h2>Utilização Automatizada</h2>

  <p>
    O <strong>Contrato Público</strong> corre num servidor pessoal com recursos limitados. Para que o
    serviço continue operacional para todos, pedimos que não extraia dados do site de forma automatizada.
  </p>

  <p>
    Como alternativa poderá utilizar o
    <a href="https://github.com/chicoferreira/contratopublico/tree/main/backend/crates/scraper"
      >scraper incluído no repositório do projeto</a
    >, a <a href="https://www.base.gov.pt/APIBase2V2">API do Portal BASE</a> diretamente, ou o
    <a
      href="https://dados.gov.pt/pt/datasets/contratos-publicos-portal-base-impic-contratos-de-2012-a-2026"
      ><em>dataset</em> de contratos públicos do Portal BASE do dados.gov</a
    >. Em qualquer dos casos, deve respeitar os termos de utilização de cada plataforma.
  </p>

  <p>
    Pedidos automatizados podem ser bloqueados sem aviso prévio. Para solicitar o desbloqueio,
    contacte-nos através de <a href="mailto:contacto@contratopublico.pt"
      >contacto@contratopublico.pt</a
    >.
  </p>

  <h2>Proteção de Dados Pessoais (RGPD)</h2>

  <h3>Dados dos Contratos</h3>

  <p>
    O cumprimento do RGPD é assegurado pelo próprio <a href="https://www.base.gov.pt/base4"
      >Portal BASE</a>
    (<a href="https://www.base.gov.pt/Base4/pt/perguntas-frequentes/"
      >ver ponto 3.11 das respetivas FAQ</a
    >). Os dados publicados no BASE já são tratados de acordo com o RGPD e demais legislação
    aplicável; o <strong>Contrato Público</strong> apenas reutiliza esses dados tal como são disponibilizados.
  </p>

  <p>
    O Portal BASE aplica as medidas de proteção necessárias, incluindo a omissão de dados pessoais
    não publicitáveis. Por exemplo, o NIF de pessoas singulares é omitido nos termos do Artigo 27.º
    da Lei n.º 58/2019, e o <strong>Contrato Público</strong> apresenta exatamente a mesma informação
    já filtrada.
  </p>

  <h3>Dados de utilização</h3>

  <p>
    Não utilizamos cookies próprios nem guardamos o conteúdo das pesquisas, salvo quando necessário
    para analisar erros.
  </p>

  <p>
    O tráfego passa pela Cloudflare, que poderá utilizar cookies para fins de segurança e
    desempenho.
  </p>

  <p>
    Cada pedido à API fica registado com o endereço IP, o país de origem, o <em>User-Agent</em> e detalhes
    técnicos do pedido (URL, data e hora, resultado e eventuais erros).
  </p>

  <p>
    Estes dados são usados para prevenir abusos e diagnosticar problemas, com base no interesse
    legítimo em garantir a segurança e disponibilidade do serviço (artigo 6.º, n.º 1, alínea f), do
    RGPD).
  </p>

  <p>
    Os registos são guardados em servidores próprios, não são partilhados com terceiros e são
    eliminados automaticamente ao fim de sete dias.
  </p>

  <p>
    Pode pedir o acesso ou a eliminação dos seus dados através do email
    <a href="mailto:contacto@contratopublico.pt">contacto@contratopublico.pt</a>.
  </p>
</ProsePage>
