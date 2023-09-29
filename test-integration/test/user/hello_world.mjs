import axios from 'axios';
import { assert } from 'chai';

describe('Hello_World', function () {
    it('should return "Hello, world!"', async function () {
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/hello_world`)
        assert.equal(res.data, 'Hello, world!')
    })

    it('should return "Hello, ${input}!"', async function () {
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/hello_world`,
            { params: { who: 'test' } }
        )
        assert.equal(res.data, 'Hello, test!')
    })
});