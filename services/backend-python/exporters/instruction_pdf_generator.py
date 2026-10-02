from weasyprint import HTML, CSS
import io


def generate_build_instructions_pdf(model_title: str, model_description: str, svg_preview: str) -> bytes:
    """
    Генерирует красивую PDF-инструкцию по сборке модели.
    """

    html_template = f"""
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>{model_title}</title>
  <style>
    body {{
      font-family: Arial, sans-serif;
      margin: 2rem;
    }}
    h1 {{
      color: #333;
    }}
    .model-card {{
      border: 1px solid #ddd;
      padding: 1rem;
      margin-bottom: 2rem;
    }}
    .svg-preview {{
      width: 100%;
      max-height: 400px;
      overflow: hidden;
    }}
    img {{
      width: 100%;
      height: auto;
    }}
    ol li {{
      margin: 0.8rem 0;
    }}
    footer {{
      margin-top: 3rem;
      text-align: center;
      font-size: 0.9em;
      color: gray;
    }}
  </style>
</head>
<body>
  <h1>🛠 Инструкция сборки: {model_title}</h1>

  <div class="model-card">
    <h2>📜 Описание модели:</h2>
    <p>{model_description}</p>
  </div>

  <div class="model-card">
    <h2>🖼 SVG Предпросмотр развёртки:</h2>
    <div class="svg-preview">{svg_preview}</div>
  </div>

  <div class="model-card">
    <h2>🧩 Этапы сборки:</h2>
    <ol>
      <li><strong>Распечатайте</strong> файл unfold.svg на плотной бумаге или картоне.</li>
      <li><strong>Вырежьте</strong> модель по контуру.</li>
      <li><strong>Сверните детали</strong>, используя линии перегиба.</li>
      <li><strong>Нанесите клей</strong> на специальные зоны (часто они помечены пунктирными линиями).</li>
      <li><strong>Склейте детали</strong> согласно направлениям стрелок.</li>
      <li><strong>Дайте полностью высохнуть</strong> и убедитесь в устойчивости конструкции.</li>
    </ol>
  </div>

  <footer>
    Сгенерировано автоматически с помощью Pepakura Next AI Tools.
  </footer>
</body>
</html>
""".strip()

    css = CSS(string="""
        @page {
            size: A4;
            margin: 1cm;
        }
        body {
            font-family: Helvetica, sans-serif;
        }
    """, media="print")

    pdf_buffer = io.BytesIO()
    HTML(string=html_template).write_pdf(pdf_buffer, stylesheets=[css])
    pdf_buffer.seek(0)
    return pdf_buffer.read()
