export type MusicId = string;
export type RoomId = string;

export type Music = {
	id: MusicId;
	title: string;
	artist: string;
	votes?: number;
};

export type Vote = {
	music_id: MusicId;
	title: string;
	artist: string;
	vote_date: Date;
	like: boolean;
};

export type Room = {
	id: RoomId;
	creation: Date;
	expiration: Date;
	user_count: number;
	active: boolean;
};
