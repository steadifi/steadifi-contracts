import Context from './utils/context';

jest.setTimeout(10000);

beforeAll(() => {
  Context.fromFile();
});
