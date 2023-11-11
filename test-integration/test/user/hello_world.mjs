import axios from 'axios';
import { assert } from 'chai';

describe('Hello_World', function () {
    it('should return "Hello, world!"', async function () {
        //arrange

        //act
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/hello_world`)
        //assert
        assert.equal(res.data, 'Hello, world!')
    })

    it('should return "Hello, ${input}!"', async function () {
        //arrange

        //act
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/hello_world`,
            { params: { who: 'test' } }
        )

        //assert
        assert.equal(res.data, 'Hello, test!')
    })
});