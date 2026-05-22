export type LocaleCode = "ru" | "en";

type Messages = Record<LocaleCode, Record<string, string>>;

export const messages: Messages = {
  ru: {
    "app.title": "Pepakura Next · прототип",
    "viewer.loadObjMtl": "Загрузить OBJ+MTL",
    "viewer.loadTextures": "Загрузить текстуры",
    "viewer.fullscreen": "Полноэкранный режим",
    "viewer.exitFullscreen": "Выйти из полноэкранного режима",
    "viewer.orbitHint": "Управление камерой: ЛКМ — вращение, колесо — масштаб, ПКМ — панорамирование",
    "layout.stage.3d": "3D",
    "layout.stage.2d": "2D",
    "layout.stage.txt": "Текст",
    "layout.stage.unfold": "Развёртка",
    "rightPanel.projectTitle": "Проект",
    "rightPanel.modelLabel": "Модель",
    "rightPanel.noModel": "Модель не загружена",
    "status.ready": "Готово"
  },
  en: {
    "app.title": "Pepakura Next · prototype",
    "viewer.loadObjMtl": "Load OBJ+MTL",
    "viewer.loadTextures": "Load Textures",
    "viewer.fullscreen": "Fullscreen",
    "viewer.exitFullscreen": "Exit Fullscreen",
    "viewer.orbitHint": "Camera: LMB – rotate, wheel – zoom, RMB – pan",
    "layout.stage.3d": "3D",
    "layout.stage.2d": "2D",
    "layout.stage.txt": "Text",
    "layout.stage.unfold": "Unfold",
    "rightPanel.projectTitle": "Project",
    "rightPanel.modelLabel": "Model",
    "rightPanel.noModel": "No model loaded",
    "status.ready": "Ready"
  }
};
