import type { Messages } from "../types";

export const ru: Messages = {
  app: {
    title: "All Audio",
    subtitle: "Конвертация аудио между популярными форматами через FFmpeg.",
  },
  language: {
    label: "Язык",
    en: "English",
    ru: "Русский",
    de: "Deutsch",
    fr: "Français",
  },
  input: {
    title: "Входной файл",
    chooseFile: "Выбрать файл",
    noFile: "Файл не выбран",
    audioFilter: "Аудио",
  },
  probe: {
    title: "Информация об источнике",
    duration: "Длительность",
    codec: "Кодек",
    sampleRate: "Частота дискретизации",
    channels: "Каналы",
    bitrate: "Битрейт",
    empty: "—",
  },
  output: {
    title: "Настройки вывода",
    format: "Формат",
    mp3Bitrate: "Битрейт MP3",
    saveTo: "Сохранить в",
    noFolder: "Папка не выбрана",
    chooseFolder: "Выбрать папку",
    filename: "Имя выходного файла",
    filenamePlaceholder: "example.mp3",
  },
  cue: {
    title: "Разделение по CUE",
    chooseFile: "Выбрать CUE-файл",
    noFile: "CUE-файл не выбран",
    filter: "CUE",
    splitCheckbox: "Разделить FLAC по CUE",
    warning:
      "Разделение по CUE экспериментальное. Проверяйте границы треков. Если CUE ссылается на несколько FLAC-файлов, выберите соответствующий входной файл.",
  },
  status: {
    label: "Статус",
    idle: "Готово",
    checkingFfmpeg: "Проверка FFmpeg…",
    probing: "Анализ файла…",
    converting: "Конвертация…",
    splitting: "Разделение треков…",
    done: "Завершено",
    error: "Ошибка",
    ffmpegBothMissing:
      "Встроенные FFmpeg и FFprobe отсутствуют или повреждены. Переустановите приложение или выполните npm run fetch-ffmpeg для dev-сборки.",
    ffmpegMissing:
      "Встроенный FFmpeg отсутствует или повреждён. Переустановите приложение или выполните npm run fetch-ffmpeg для dev-сборки.",
    ffprobeMissing:
      "Встроенный FFprobe отсутствует или повреждён. Переустановите приложение или выполните npm run fetch-ffmpeg для dev-сборки.",
  },
  actions: {
    convert: "Конвертировать",
    splitTracks: "Разделить треки",
  },
  success: {
    savedTo: "Сохранено в {path}",
    createdTracks: "Создано треков: {count}.",
    createdFiles: "Создано файлов: {count}.",
  },
  errors: {
    unknown: "Неизвестная ошибка",
    commandNotFound: "Встроенный FFmpeg не удалось запустить",
    commandFailed: "Не удалось выполнить внешнюю команду",
    ffprobeFailed: "FFprobe не смог проанализировать входной файл",
    ffprobeParseFailed: "Не удалось разобрать вывод FFprobe",
    sameInputOutput: "Входной и выходной файлы должны отличаться",
    cueRequiresFlac: "Разделение по CUE поддерживается только для FLAC",
    cuePathRequired: "Не указан путь к CUE-файлу",
    outputFileExists: "Выходной файл уже существует",
    ffmpegFailed: "Ошибка конвертации FFmpeg",
    cueReadFailed: "Не удалось прочитать CUE-файл",
    cueMultipleFiles:
      "CUE ссылается на несколько аудиофайлов. Выберите соответствующий FLAC для разделения.",
    cueNoTracks: "В CUE нет треков с INDEX 01",
    invalidInputPath: "Некорректный путь к входному файлу",
    inputNotFound: "Входной файл не существует",
    unsupportedInputExtension: "Неподдерживаемое расширение входного файла",
    unsupportedOutputFormat: "Неподдерживаемый выходной формат",
    invalidOutputDir: "Некорректная выходная папка",
    outputDirNotFound: "Выходная папка не существует",
    invalidOutputFilename: "Некорректное имя выходного файла",
    outputExtensionMismatch:
      "Расширение имени файла не совпадает с выбранным форматом",
    invalidCuePath: "Некорректный путь к CUE-файлу",
    cueNotFound: "CUE-файл не существует",
    invalidCueExtension: "CUE-файл должен иметь расширение .cue",
    invalidMp3Bitrate: "Неподдерживаемый битрейт MP3",
    conversionTaskFailed: "Ошибка задачи конвертации",
    cueFileMismatch:
      'В CUE нет треков для «{name}». Указанные файлы: {files}',
    cueAmbiguousFile: 'В CUE несколько секций для «{name}»',
    cueInvalidTrackOrder:
      "Трек {trackB} начинается одновременно с треком {trackA} или раньше него. Проверьте INDEX 01 в CUE.",
    cueInvalidTrackDuration:
      "У трека {trackA} некорректная длительность относительно трека {trackB}",
    bundledFfmpegMissing: "Встроенный FFmpeg не найден в пакете приложения",
    bundledFfprobeMissing: "Встроенный FFprobe не найден в пакете приложения",
    bundledFfmpegNotExecutable: "Встроенный FFmpeg не является исполняемым",
    bundledFfmpegInvalid: "Встроенный FFmpeg повреждён или недоступен",
    bundledFfmpegResolveFailed: "Не удалось найти встроенные ресурсы FFmpeg",
  },
};
