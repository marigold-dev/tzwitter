export interface CollectedData {
  level: number;
  mintableDate: Date;
}

export interface Tweet {
  id: number;
  author: string;
  content: string;
  likes: number;
  isLiked: boolean;
  collected?: CollectedData;
}
