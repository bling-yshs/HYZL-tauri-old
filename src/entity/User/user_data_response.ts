import {User} from "./user.ts";

export interface UserDataResponse {
    code: number;
    data: User;
    message: string;
}