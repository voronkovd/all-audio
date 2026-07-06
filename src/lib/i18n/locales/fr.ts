import type { Messages } from "../types";

export const fr: Messages = {
  app: {
    title: "All Audio",
    subtitle:
      "Convertissez des fichiers audio entre les formats courants avec FFmpeg.",
  },
  language: {
    label: "Langue",
    en: "English",
    ru: "Русский",
    de: "Deutsch",
    fr: "Français",
  },
  input: {
    title: "Fichier source",
    chooseFile: "Choisir un fichier",
    noFile: "Aucun fichier sélectionné",
    audioFilter: "Audio",
  },
  probe: {
    title: "Informations source",
    duration: "Durée",
    codec: "Codec",
    sampleRate: "Fréquence d'échantillonnage",
    channels: "Canaux",
    bitrate: "Débit",
    empty: "—",
  },
  output: {
    title: "Paramètres de sortie",
    format: "Format",
    mp3Bitrate: "Débit MP3",
    saveTo: "Enregistrer dans",
    noFolder: "Aucun dossier sélectionné",
    chooseFolder: "Choisir un dossier",
    filename: "Nom du fichier de sortie",
    filenamePlaceholder: "exemple.mp3",
  },
  cue: {
    title: "Découpage CUE",
    chooseFile: "Choisir un fichier CUE",
    noFile: "Aucun fichier CUE sélectionné",
    filter: "CUE",
    splitCheckbox: "Découper le FLAC par CUE",
    warning:
      "Le découpage CUE est expérimental. Vérifiez les limites des pistes. Si la feuille CUE référence plusieurs fichiers FLAC, sélectionnez le fichier source correspondant.",
  },
  status: {
    label: "Statut",
    idle: "Prêt",
    checkingFfmpeg: "Vérification de FFmpeg…",
    probing: "Analyse du fichier…",
    converting: "Conversion…",
    splitting: "Découpage des pistes…",
    done: "Terminé",
    error: "Erreur",
    ffmpegBothMissing:
      "FFmpeg et FFprobe intégrés sont manquants ou endommagés. Réinstallez l'application ou exécutez npm run fetch-ffmpeg pour le développement.",
    ffmpegMissing:
      "FFmpeg intégré manquant ou endommagé. Réinstallez l'application ou exécutez npm run fetch-ffmpeg pour le développement.",
    ffprobeMissing:
      "FFprobe intégré manquant ou endommagé. Réinstallez l'application ou exécutez npm run fetch-ffmpeg pour le développement.",
  },
  actions: {
    convert: "Convertir",
    splitTracks: "Découper les pistes",
  },
  success: {
    savedTo: "Enregistré dans {path}",
    createdTracks: "{count} pistes créées.",
    createdFiles: "{count} fichiers créés.",
  },
  errors: {
    unknown: "Erreur inconnue",
    commandNotFound: "Impossible d'exécuter FFmpeg intégré",
    commandFailed: "Échec de l'exécution de la commande externe",
    ffprobeFailed: "FFprobe n'a pas pu analyser le fichier source",
    ffprobeParseFailed: "Impossible d'analyser la sortie de FFprobe",
    sameInputOutput:
      "Les fichiers source et de sortie doivent être différents",
    cueRequiresFlac: "Le découpage CUE est pris en charge uniquement pour FLAC",
    cuePathRequired: "Le chemin du fichier CUE est requis",
    outputFileExists: "Le fichier de sortie existe déjà",
    ffmpegFailed: "Échec de la conversion FFmpeg",
    cueReadFailed: "Impossible de lire le fichier CUE",
    cueMultipleFiles:
      "Le CUE référence plusieurs fichiers audio. Sélectionnez le FLAC correspondant.",
    cueNoTracks: "Le CUE ne contient aucune piste avec INDEX 01",
    invalidInputPath: "Chemin du fichier source invalide",
    inputNotFound: "Le fichier source n'existe pas",
    unsupportedInputExtension: "Extension du fichier source non prise en charge",
    unsupportedOutputFormat: "Format de sortie non pris en charge",
    invalidOutputDir: "Dossier de sortie invalide",
    outputDirNotFound: "Le dossier de sortie n'existe pas",
    invalidOutputFilename: "Nom de fichier de sortie invalide",
    outputExtensionMismatch:
      "L'extension du nom de fichier ne correspond pas au format choisi",
    invalidCuePath: "Chemin du fichier CUE invalide",
    cueNotFound: "Le fichier CUE n'existe pas",
    invalidCueExtension: "Le fichier CUE doit avoir l'extension .cue",
    invalidMp3Bitrate: "Débit MP3 non pris en charge",
    conversionTaskFailed: "Échec de la tâche de conversion",
    cueFileMismatch:
      'Le CUE ne contient pas de pistes pour « {name} ». Fichiers référencés : {files}',
    cueAmbiguousFile: 'Le CUE contient plusieurs sections pour « {name} »',
    cueInvalidTrackOrder:
      "La piste {trackB} commence en même temps ou avant la piste {trackA}. Vérifiez INDEX 01 dans le CUE.",
    cueInvalidTrackDuration:
      "La piste {trackA} a une durée invalide par rapport à la piste {trackB}",
    bundledFfmpegMissing: "FFmpeg intégré introuvable dans le paquet de l'application",
    bundledFfprobeMissing: "FFprobe intégré introuvable dans le paquet de l'application",
    bundledFfmpegNotExecutable: "FFmpeg intégré n'est pas exécutable",
    bundledFfmpegInvalid: "FFmpeg intégré invalide ou indisponible",
    bundledFfmpegResolveFailed: "Impossible de localiser les ressources FFmpeg intégrées",
  },
};
