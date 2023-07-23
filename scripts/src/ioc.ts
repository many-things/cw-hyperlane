import { Container } from 'inversify';

export const CONTAINER = new Container({ autoBindInjectable: true, defaultScope: 'Singleton' });
export const TYPES = {};

// referenced by tsoa
export const iocContainer = CONTAINER;
