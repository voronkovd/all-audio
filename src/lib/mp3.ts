export interface Mp3Tags {
  title: string;
  artist: string;
  album: string;
  albumArtist: string;
  year: string;
  genre: string;
  track: string;
}

export interface Mp3TagsApi {
  title?: string;
  artist?: string;
  album?: string;
  album_artist?: string;
  year?: string;
  genre?: string;
  track?: string;
}

export function emptyMp3Tags(): Mp3Tags {
  return {
    title: "",
    artist: "",
    album: "",
    albumArtist: "",
    year: "",
    genre: "",
    track: "",
  };
}

export function mp3TagsToApi(tags: Mp3Tags): Mp3TagsApi | undefined {
  const api: Mp3TagsApi = {
    title: tags.title.trim() || undefined,
    artist: tags.artist.trim() || undefined,
    album: tags.album.trim() || undefined,
    album_artist: tags.albumArtist.trim() || undefined,
    year: tags.year.trim() || undefined,
    genre: tags.genre.trim() || undefined,
    track: tags.track.trim() || undefined,
  };

  if (Object.values(api).every((value) => value === undefined)) {
    return undefined;
  }

  return api;
}

export function mp3TagsFromMetadata(metadata: {
  title?: string | null;
  artist?: string | null;
  album?: string | null;
  album_artist?: string | null;
  year?: string | null;
  genre?: string | null;
  track?: string | null;
}): Mp3Tags {
  return {
    title: metadata.title ?? "",
    artist: metadata.artist ?? "",
    album: metadata.album ?? "",
    albumArtist: metadata.album_artist ?? "",
    year: metadata.year ?? "",
    genre: metadata.genre ?? "",
    track: metadata.track ?? "",
  };
}

export function fillEmptyMp3Tags(current: Mp3Tags, incoming: Partial<Mp3Tags>): Mp3Tags {
  const fill = (currentValue: string, incomingValue?: string) => {
    if (currentValue.trim().length > 0) {
      return currentValue;
    }
    return incomingValue?.trim() ?? "";
  };

  return {
    title: fill(current.title, incoming.title),
    artist: fill(current.artist, incoming.artist),
    album: fill(current.album, incoming.album),
    albumArtist: fill(current.albumArtist, incoming.albumArtist),
    year: fill(current.year, incoming.year),
    genre: fill(current.genre, incoming.genre),
    track: fill(current.track, incoming.track),
  };
}
